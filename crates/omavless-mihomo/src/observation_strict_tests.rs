// SPDX-License-Identifier: MIT
use super::*;
use std::os::unix::fs::symlink;

struct Fixture(std::path::PathBuf);
impl Fixture {
    fn new() -> Self {
        Self(crate::test_temp::directory("strict-observation").unwrap())
    }
    fn process(&self, pid: &str, comm: &[u8]) {
        fs::create_dir(self.0.join(pid)).unwrap();
        fs::write(self.0.join(pid).join("comm"), comm).unwrap();
    }
    fn interface(&self, name: &str, tun: bool) {
        fs::create_dir(self.0.join(name)).unwrap();
        if tun {
            fs::write(self.0.join(name).join("tun_flags"), "0x1001\n").unwrap();
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn strict_processes_match_exactly_without_rejecting_unicode_or_spaces() {
    let f = Fixture::new();
    f.process("1", b"mihomo\n");
    f.process("2", "worker поток\n".as_bytes());
    f.process("3", b"mihomo-helper\n");
    f.process("4", &[255, b'\n']);
    f.process("5", b"\n");
    fs::write(f.0.join("meminfo"), b"ignored").unwrap();
    assert_eq!(
        processes_named_strict(&f.0, "mihomo").unwrap(),
        BTreeSet::from([1])
    );
    assert!(processes_named_strict(&f.0, "other").unwrap().is_empty());
    assert!(processes_named_strict(&f.0, "../bad").is_err());
}

#[test]
fn strict_process_boundaries_and_incomplete_scans_refuse() {
    let f = Fixture::new();
    assert!(processes_named_strict(&f.0.join("missing"), "mihomo").is_err());
    f.process("1", b"mihomo\n");
    f.process("2", b"mihomo\n");
    assert_eq!(
        processes_named_strict_bounded(&f.0, "mihomo", 2, 2)
            .unwrap()
            .len(),
        2
    );
    assert!(processes_named_strict_bounded(&f.0, "mihomo", 1, 2).is_err());
    assert!(processes_named_strict_bounded(&f.0, "mihomo", 2, 1).is_err());
    fs::remove_file(f.0.join("2/comm")).unwrap();
    assert!(processes_named_strict(&f.0, "mihomo").is_err());
}

const TASK_PHASES: [ProcessScanPhase; 7] = [
    ProcessScanPhase::Directory,
    ProcessScanPhase::CommMetadata,
    ProcessScanPhase::Open,
    ProcessScanPhase::OpenedMetadata,
    ProcessScanPhase::Read,
    ProcessScanPhase::AfterDirectory,
    ProcessScanPhase::AfterComm,
];

#[test]
fn vanished_task_at_each_phase_does_not_poison_complete_inventory() {
    for phase in TASK_PHASES {
        let f = Fixture::new();
        f.process("1", b"mihomo\n");
        f.process("2", b"short worker\n");
        let mut injected = false;
        let found = processes_named_strict_scan(&f.0, "mihomo", 16, 16, |step, directory| {
            if step == phase && directory.file_name().unwrap() == "2" {
                fs::remove_dir_all(directory).unwrap();
                injected = true;
            }
            Ok(())
        });
        assert!(injected);
        assert_eq!(found, Ok(BTreeSet::from([1])), "{phase:?}");
    }
}

#[test]
fn task_lookup_errors_require_independent_directory_disappearance() {
    for phase in TASK_PHASES {
        for errno in [nix::libc::ENOENT, nix::libc::ESRCH] {
            for vanished in [false, true] {
                let f = Fixture::new();
                f.process("1", b"worker\n");
                let result =
                    processes_named_strict_scan(&f.0, "mihomo", 16, 16, |step, directory| {
                        if step == phase {
                            if vanished {
                                fs::remove_dir_all(directory).unwrap();
                            }
                            return Err(io::Error::from_raw_os_error(errno));
                        }
                        Ok(())
                    });
                // Initial metadata ENOENT is already the directory absence
                // lookup. ESRCH is NOT accepted at that initial phase.
                let permitted = if phase == ProcessScanPhase::Directory {
                    errno == nix::libc::ENOENT
                } else {
                    vanished
                };
                assert_eq!(result.is_ok(), permitted, "{phase:?}/{errno}/{vanished}");
            }
        }
    }
}

#[test]
fn task_permission_and_other_errors_never_become_absence() {
    for phase in TASK_PHASES {
        for errno in [nix::libc::EACCES, nix::libc::EPERM, nix::libc::EIO] {
            let f = Fixture::new();
            f.process("1", b"worker\n");
            let result = processes_named_strict_scan(&f.0, "mihomo", 16, 16, |step, directory| {
                if step == phase {
                    fs::remove_dir_all(directory).unwrap();
                    return Err(io::Error::from_raw_os_error(errno));
                }
                Ok(())
            });
            assert!(result.is_err(), "{phase:?}/{errno}");
        }
    }
}

#[test]
fn live_missing_comm_at_lookup_phases_still_refuses() {
    for phase in [
        ProcessScanPhase::CommMetadata,
        ProcessScanPhase::Open,
        ProcessScanPhase::AfterComm,
    ] {
        let f = Fixture::new();
        f.process("1", b"worker\n");
        let result = processes_named_strict_scan(&f.0, "mihomo", 16, 16, |step, directory| {
            if step == phase {
                fs::remove_file(directory.join("comm")).unwrap();
            }
            Ok(())
        });
        assert!(result.is_err(), "{phase:?}");
    }
}

#[test]
fn pid_or_comm_replacement_is_not_process_disappearance() {
    for replace_directory in [false, true] {
        let f = Fixture::new();
        f.process("1", b"worker\n");
        let result = processes_named_strict_scan(&f.0, "mihomo", 16, 16, |step, directory| {
            if step == ProcessScanPhase::AfterDirectory {
                if replace_directory {
                    // Keep the old inode alive; allocator reuse cannot make
                    // the synthetic replacement nondeterministic.
                    fs::rename(directory, f.0.join("retired")).unwrap();
                    fs::create_dir(directory).unwrap();
                } else {
                    fs::rename(directory.join("comm"), directory.join("old-comm")).unwrap();
                }
                fs::write(directory.join("comm"), b"mihomo\n").unwrap();
            }
            Ok(())
        });
        assert!(result.is_err());
    }
}

#[test]
fn missing_or_replaced_proc_root_cannot_fabricate_empty_inventory() {
    for when in [ProcessScanPhase::Directory, ProcessScanPhase::RootRecheck] {
        for replacement in [false, true] {
            let f = Fixture::new();
            let root = f.0.join("proc");
            fs::create_dir(&root).unwrap();
            fs::create_dir(root.join("1")).unwrap();
            fs::write(root.join("1/comm"), b"worker\n").unwrap();
            let result = processes_named_strict_scan(&root, "mihomo", 16, 16, |step, _| {
                if step == when {
                    fs::rename(&root, f.0.join("retired")).unwrap();
                    if replacement {
                        fs::create_dir(&root).unwrap();
                    }
                }
                Ok(())
            });
            assert!(result.is_err(), "{when:?}/{replacement}");
        }
    }
}

#[test]
fn vanished_tasks_do_not_exempt_scan_bounds_or_later_invalid_tasks() {
    let f = Fixture::new();
    f.process("1", b"worker\n");
    f.process("2", b"worker\n");
    assert!(
        processes_named_strict_scan(&f.0, "mihomo", 1, 16, |step, directory| {
            if step == ProcessScanPhase::Directory {
                fs::remove_dir_all(directory).unwrap();
            }
            Ok(())
        })
        .is_err()
    );

    let f = Fixture::new();
    f.process("1", b"worker\n");
    f.process("2", b"invalid comm without newline");
    assert!(
        processes_named_strict_scan(&f.0, "mihomo", 16, 16, |step, directory| {
            if step == ProcessScanPhase::Directory && directory.file_name().unwrap() == "1" {
                fs::remove_dir_all(directory).unwrap();
            }
            Ok(())
        })
        .is_err()
    );
}

#[cfg(target_os = "linux")]
#[test]
fn exited_harmless_child_comm_descriptor_reports_proven_task_disappearance() {
    struct ChildGuard(std::process::Child);
    impl Drop for ChildGuard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut child = ChildGuard(
        std::process::Command::new("/usr/bin/sleep")
            .arg("60")
            .spawn()
            .unwrap(),
    );
    let directory = std::path::PathBuf::from(format!("/proc/{}", child.0.id()));
    let mut comm = fs::File::open(directory.join("comm")).unwrap();
    child.0.kill().unwrap();
    child.0.wait().unwrap();
    let mut raw = Vec::new();
    let read = comm.read_to_end(&mut raw);
    assert_eq!(
        read.as_ref().unwrap_err().raw_os_error(),
        Some(nix::libc::ESRCH)
    );
    assert_eq!(task_step(&directory, read), Ok(None));
}

#[test]
fn strict_comm_invalid_bytes_size_and_numeric_identity_refuse() {
    for raw in [
        vec![],
        vec![0, b'\n'],
        b"missing newline".to_vec(),
        vec![b'x'; 65],
    ] {
        let f = Fixture::new();
        f.process("1", &raw);
        assert!(processes_named_strict(&f.0, "mihomo").is_err());
    }
    for pid in ["0", "01", "4294967296"] {
        let f = Fixture::new();
        f.process(pid, b"other\n");
        assert!(processes_named_strict(&f.0, "mihomo").is_err());
    }
    let f = Fixture::new();
    let mut legal = vec![b'x'; 63];
    legal.push(b'\n');
    f.process("1", &legal);
    assert!(processes_named_strict(&f.0, "mihomo").unwrap().is_empty());
}

#[test]
fn strict_proc_symlink_and_nonregular_comm_refuse_without_opening() {
    let f = Fixture::new();
    f.process("1", b"mihomo\n");
    symlink(f.0.join("1"), f.0.join("2")).unwrap();
    assert!(processes_named_strict(&f.0, "mihomo").is_err());
    fs::remove_file(f.0.join("2")).unwrap();
    fs::remove_file(f.0.join("1/comm")).unwrap();
    symlink("/dev/zero", f.0.join("1/comm")).unwrap();
    assert!(processes_named_strict(&f.0, "mihomo").is_err());
    fs::remove_file(f.0.join("1/comm")).unwrap();
    fs::create_dir(f.0.join("1/comm")).unwrap();
    assert!(processes_named_strict(&f.0, "mihomo").is_err());
}

#[test]
fn strict_net_empty_regular_and_realistic_interface_symlinks() {
    let f = Fixture::new();
    assert_eq!(tun_interface_count_strict(&f.0), Ok(0));
    f.interface("lo", false);
    f.interface("tun", true);
    assert_eq!(tun_interface_count_strict(&f.0), Ok(1));
    let target = Fixture::new();
    target.interface("eth", false);
    symlink(target.0.join("eth"), f.0.join("eth")).unwrap();
    assert_eq!(tun_interface_count_strict(&f.0), Ok(1));
    fs::remove_dir(target.0.join("eth")).unwrap();
    assert!(tun_interface_count_strict(&f.0).is_err());
}

#[test]
fn strict_net_bounds_flags_and_non_directory_refuse() {
    let f = Fixture::new();
    f.interface("a", true);
    f.interface("b", true);
    assert_eq!(tun_interface_count_strict_bounded(&f.0, 2, 2), Ok(2));
    assert!(tun_interface_count_strict_bounded(&f.0, 1, 2).is_err());
    assert!(tun_interface_count_strict_bounded(&f.0, 2, 1).is_err());
    for raw in [b"broken".as_slice(), &[255], &[b'x'; 33]] {
        fs::write(f.0.join("b/tun_flags"), raw).unwrap();
        assert!(tun_interface_count_strict(&f.0).is_err());
    }
    fs::remove_file(f.0.join("b/tun_flags")).unwrap();
    symlink(f.0.join("a/tun_flags"), f.0.join("b/tun_flags")).unwrap();
    assert!(tun_interface_count_strict(&f.0).is_err());
    let bad = Fixture::new();
    fs::write(bad.0.join("not-interface"), b"").unwrap();
    assert!(tun_interface_count_strict(&bad.0).is_err());
    assert!(tun_interface_count_strict(&bad.0.join("missing")).is_err());
}

#[test]
fn strict_errors_never_include_host_details() {
    assert_eq!(
        StrictObservationError.to_string(),
        "Host inventory could not be verified"
    );
    assert_eq!(
        format!("{StrictObservationError:?}"),
        "StrictObservationError"
    );
}
