// SPDX-License-Identifier: MIT

use std::fs;
use std::path::PathBuf;

#[test]
fn packaged_user_unit_preserves_arch_file_capability_contract() {
    let unit = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packaging/systemd/omavless-runtime.service");
    let text = fs::read_to_string(unit).unwrap();
    assert!(text.contains("\nExecStart=/usr/bin/omavless daemon\n"));
    assert!(text.contains("\nConditionFileIsExecutable=/usr/bin/omavless\n"));
    assert!(text.contains("\nRuntimeDirectory=omavless\n"));
    assert!(text.contains("\nRuntimeDirectoryMode=0700\n"));
    assert!(text.contains("\nRuntimeDirectoryPreserve=yes\n"));
    assert!(text.contains("\nRequires=omavless-login-prepare.service\n"));
    assert!(text.contains("\nAfter=omavless-login-prepare.service\n"));
    assert!(text.contains("\nUMask=0077\n"));
    assert!(text.contains("\nNoNewPrivileges=no\n"));
    assert!(text.contains("\nLimitCORE=0\n"));
    // These user-service directives conflict with host file capabilities:
    // mount isolation creates a user namespace; seccomp restrictions can
    // implicitly set NoNewPrivs even with an explicit `no` above.
    for forbidden in [
        "PrivateTmp=",
        "ProtectSystem=",
        "ProtectHome=",
        "PrivateUsers=",
        "RestrictRealtime=",
        "LockPersonality=",
        "SystemCallFilter=",
        "AmbientCapabilities=",
        "ExecStartPre=",
    ] {
        assert!(!text.lines().any(|line| line.starts_with(forbidden)));
    }
    for directive in [
        "ConfigurationDirectory=omavless",
        "ConfigurationDirectoryMode=0700",
        "StateDirectory=omavless",
        "StateDirectoryMode=0700",
        "CacheDirectory=omavless",
        "CacheDirectoryMode=0700",
    ] {
        assert!(text.lines().any(|line| line == directive));
    }
    assert!(!text.contains("python"));
    assert!(!text.contains("/bin/sh"));
    assert!(!text.contains("sudo"));
    assert!(!text.contains("pkexec"));
}

#[test]
fn login_unit_is_fixed_oneshot_not_a_second_runtime_or_restart_hook() {
    let package = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packaging/arch/PKGBUILD.local.in"),
    )
    .unwrap();
    assert!(
        package
            .lines()
            .any(|line| line.starts_with("depends=(") && line.contains("'bubblewrap'"))
    );
    let text = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packaging/systemd/omavless-login-prepare.service"),
    )
    .unwrap();
    for required in [
        "Type=oneshot",
        "RemainAfterExit=yes",
        "ExecCondition=/usr/bin/omavless login-condition",
        "ExecStart=/usr/bin/omavless login-prepare",
        "RuntimeDirectory=omavless",
        "RuntimeDirectoryMode=0700",
        "RuntimeDirectoryPreserve=yes",
        "UMask=0077",
        "TimeoutStartSec=60s",
        "Before=omavless-runtime.service",
    ] {
        assert!(text.lines().any(|line| line == required));
    }
    for forbidden in [
        "PartOf=",
        "Restart=",
        "ExecStop=",
        "ExecStartPre=",
        "ConditionPath",
        "ConditionFile",
        "[Install]",
        "sudo",
        "pkexec",
        "/bin/sh",
        "python",
    ] {
        assert!(!text.contains(forbidden));
    }
}
