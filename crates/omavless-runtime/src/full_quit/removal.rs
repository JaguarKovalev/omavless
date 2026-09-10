// SPDX-License-Identifier: MIT

//! Fixed Omarchy unload observer. UI destruction alone is never a shutdown
//! request. Only a subsequent, explicit disabled/removed registry observation
//! admits the existing fenced full-Quit runtime path. No plugin files are used
//! after launch, no packaged units are deleted, and there is no retry against a
//! new desired revision. Shell and daemon state are not one atomic transaction:
//! the final registry observation is the admission point (see acceptance doc).

use super::*;
use std::io;
use std::path::{Component, PathBuf};

const GRACE: Duration = Duration::from_secs(2);
const FAILURE: &str = "OmaVLESS shutdown was not verified. Re-enable the plugin, check connection state and use Quit. VPN may still be running.";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Registry {
    Enabled,
    Disabled,
    Absent,
    Unknown,
}

fn registry(text: &str) -> Registry {
    #[derive(serde::Deserialize)]
    struct Row {
        id: String,
        enabled: bool,
    }
    // Deriving named fields rejects duplicate id/enabled keys too. Extra shell
    // metadata is ignored, never logged or passed to a mutation.
    let Ok(rows) = serde_json::from_str::<Vec<Row>>(text) else {
        return Registry::Unknown;
    };
    if rows.len() > 1024 {
        return Registry::Unknown;
    }
    let mut result = Registry::Absent;
    for row in rows {
        // A malformed projection is not evidence that an omitted plugin was
        // removed. Validate even unrelated row identity/boolean, ignore names.
        if row.id == PLUGIN {
            if result != Registry::Absent {
                return Registry::Unknown;
            }
            result = if row.enabled {
                Registry::Enabled
            } else {
                Registry::Disabled
            };
        }
    }
    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Manifest {
    Present,
    Absent,
    Unknown,
}

fn manifest_at(home: &Path, uid: u32) -> Manifest {
    if !home.is_absolute() {
        return Manifest::Unknown;
    }
    // This is Omarchy's actual fixed install root, not arbitrary XDG config.
    let path = home.join(".config/omarchy/plugins/kdk.omavless/manifest.json");
    if path
        .components()
        .any(|component| !matches!(component, Component::RootDir | Component::Normal(_)))
    {
        return Manifest::Unknown;
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::Normal(_) => current.push(component.as_os_str()),
            _ => return Manifest::Unknown,
        }
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Manifest::Absent,
            Err(_) => return Manifest::Unknown,
        };
        if metadata.file_type().is_symlink()
            || (metadata.uid() != 0 && metadata.uid() != uid)
            || metadata.mode() & 0o022 != 0
        {
            return Manifest::Unknown;
        }
        if current == path {
            return if metadata.is_file() {
                Manifest::Present
            } else {
                Manifest::Unknown
            };
        }
        if !metadata.is_dir() {
            return Manifest::Unknown;
        }
    }
    Manifest::Unknown
}

fn cleanup_required(registry: Registry, manifest: Manifest) -> bool {
    // Enabled always wins, including during a replace/rescan. Unknown shell
    // state never means removal, even if the manifest also looks absent.
    registry == Registry::Disabled || (registry == Registry::Absent && manifest == Manifest::Absent)
}

fn installed_cleanup_required() -> bool {
    let state = omarchy_command(&["plugin", "list", "--json"])
        .map(|text| registry(&text))
        .unwrap_or(Registry::Unknown);
    let manifest = std::env::var_os("HOME")
        .map(|home| manifest_at(Path::new(&home), Uid::current().as_raw()))
        .unwrap_or(Manifest::Unknown);
    cleanup_required(state, manifest)
}

// The watch policy is injectable only inside this private test boundary, not
// via CLI arguments/environment, and does not duplicate lifecycle decisions.
trait WatchHost: Host {
    fn authenticate_watch(&mut self) -> Result<()>;
    fn snapshot_fence(&mut self) -> Result<(String, u64)>;
    fn grace(&mut self);
    fn cleanup_required(&mut self) -> bool;
}

fn watch_host(host: &mut impl WatchHost, operation: &str) -> Result<bool> {
    host.authenticate_watch()?;
    let (instance, revision) = host.snapshot_fence()?;
    if !arguments_valid(&instance, revision, operation) {
        return Err(FullQuitError::PreconditionsFailed);
    }
    host.grace();
    if !host.cleanup_required() {
        return Ok(false);
    }
    // Each unary client consumes its connection. Authenticate the new peer,
    // but retain the original instance/revision; stale/busy never auto-retry.
    if !host.cleanup_required() {
        return Ok(false);
    }
    // Do not hold an idle unary stream through a shell query (10s bound versus
    // the daemon's 5s framing deadline). This last registry observation admits
    // the action; re-authenticate and immediately send the original fence.
    host.authenticate_watch()?;
    finish_runtime(host, &instance, revision, operation)?;
    Ok(true)
}

impl WatchHost for InstalledHost {
    fn authenticate_watch(&mut self) -> Result<()> {
        self.authenticate(false)
    }
    fn snapshot_fence(&mut self) -> Result<(String, u64)> {
        let response = crate::call_stream_with_timeout(
            self.stream
                .take()
                .ok_or(FullQuitError::PreconditionsFailed)?,
            Uid::current().as_raw(),
            "system.hello",
            json!({"versions":[1]}),
            SERVICE_QUERY_TIMEOUT,
        )
        .map_err(|_| FullQuitError::PreconditionsFailed)?;
        if response["ok"] != true {
            return Err(FullQuitError::PreconditionsFailed);
        }
        Ok((
            response["result"]["instanceId"]
                .as_str()
                .ok_or(FullQuitError::PreconditionsFailed)?
                .to_owned(),
            response["revision"]
                .as_u64()
                .ok_or(FullQuitError::PreconditionsFailed)?,
        ))
    }
    fn grace(&mut self) {
        std::thread::sleep(GRACE);
    }
    fn cleanup_required(&mut self) -> bool {
        installed_cleanup_required()
    }
}

/// Fixed zero-argument command called directly from the installed binary by
/// native QML destruction. A private nonblocking lock coalesces simultaneous
/// monitors/watchers; queued stale unload work must never capture fresh intent.
pub fn run() -> std::result::Result<bool, &'static str> {
    let result = run_inner();
    if result.is_err() && installed_cleanup_required() {
        // Best-effort fixed public recovery notice: no raw child output, store
        // content or arbitrary command can reach the desktop notification.
        let mut command = Command::new("/usr/bin/notify-send");
        command.args([
            "--app-name=OmaVLESS",
            "--urgency=critical",
            "OmaVLESS",
            FAILURE,
        ]);
        let _ = bounded_fixed_query(command, Duration::from_secs(3));
    }
    result.map_err(|_| FAILURE)
}

fn run_inner() -> Result<bool> {
    let error = FullQuitError::PreconditionsFailed;
    if crate::frontend_bridge::current_plugin_target().map_err(|_| error)?
        == crate::cutover_transaction::BridgeTarget::Legacy
    {
        return Ok(false);
    }
    let paths = RuntimePaths::current().map_err(|_| error)?;
    crate::validate_client_directory(&paths.directory, Uid::current().as_raw())
        .map_err(|_| error)?;
    let _watch = match OwnerLock::acquire(
        &paths.directory.join("plugin-removal.lock"),
        Uid::current().as_raw(),
    ) {
        Ok(lock) => lock,
        Err(crate::RuntimeError::AlreadyRunning) => return Ok(false),
        Err(_) => return Err(error),
    };
    // Full Quit itself unloads QML after verified shutdown. Do not attempt to
    // reconnect/restart the disabled daemon from that second destructor event.
    // This no-op is not an additional claim of empty-host verification.
    if stopped_unit() && installed_unit(true) {
        return Ok(false);
    }
    let mut host = InstalledHost {
        paths,
        stream: None,
        owner_lock: None,
    };
    watch_host(&mut host, &format!("plugin-removal-{}", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn only_proven_disable_or_complete_removal_admits_cleanup() {
        for state in [
            Registry::Enabled,
            Registry::Disabled,
            Registry::Absent,
            Registry::Unknown,
        ] {
            for file in [Manifest::Present, Manifest::Absent, Manifest::Unknown] {
                assert_eq!(
                    cleanup_required(state, file),
                    state == Registry::Disabled
                        || (state == Registry::Absent && file == Manifest::Absent)
                );
            }
        }
    }

    #[test]
    fn registry_rejects_ambiguous_projection_without_echoing_names() {
        assert_eq!(registry("[]"), Registry::Absent);
        assert_eq!(
            registry(r#"[{"id":"other","enabled":true}]"#),
            Registry::Absent
        );
        assert_eq!(
            registry(r#"[{"id":"kdk.omavless","enabled":false}]"#),
            Registry::Disabled
        );
        assert_eq!(
            registry(r#"[{"id":"kdk.omavless","enabled":true}]"#),
            Registry::Enabled
        );
        for text in [
            "private input",
            "{}",
            "[null]",
            r#"[{"id":"other"}]"#,
            r#"[{"id":"kdk.omavless","enabled":true,"enabled":false}]"#,
            r#"[{"id":"kdk.omavless","enabled":false},{"id":"kdk.omavless","enabled":false}]"#,
        ] {
            assert_eq!(registry(text), Registry::Unknown);
        }
    }

    struct Fake {
        calls: Vec<&'static str>,
        decisions: VecDeque<bool>,
        fail_at: Option<usize>,
    }
    impl Fake {
        fn step(&mut self, step: &'static str) -> Result<()> {
            self.calls.push(step);
            if self.fail_at == Some(self.calls.len() - 1) {
                Err(FullQuitError::RequestFailed)
            } else {
                Ok(())
            }
        }
    }
    impl Host for Fake {
        fn preflight(&mut self) -> Result<()> {
            panic!("must not require enabled plugin")
        }
        fn request_shutdown(
            &mut self,
            instance: &str,
            revision: u64,
            operation: &str,
        ) -> Result<()> {
            assert_eq!(
                (instance, revision, operation),
                ("original-instance", 7, "one-operation")
            );
            self.step("quit")
        }
        fn wait_and_lock(&mut self) -> Result<()> {
            self.step("owner-lock")
        }
        fn verify_stopped(&mut self) -> Result<()> {
            self.step("empty")
        }
        fn disable_runtime(&mut self) -> Result<()> {
            self.step("disable-runtime")
        }
        fn verify_runtime_disabled(&mut self) -> Result<()> {
            self.step("disabled")
        }
        fn disable_plugin(&mut self) -> Result<()> {
            panic!("already unloaded")
        }
        fn verify_plugin_disabled(&mut self) -> Result<()> {
            panic!("not full Quit")
        }
    }
    impl WatchHost for Fake {
        fn authenticate_watch(&mut self) -> Result<()> {
            self.step("authenticate")
        }
        fn snapshot_fence(&mut self) -> Result<(String, u64)> {
            self.step("snapshot")?;
            Ok(("original-instance".into(), 7))
        }
        fn grace(&mut self) {
            self.calls.push("grace");
        }
        fn cleanup_required(&mut self) -> bool {
            self.calls.push("observe");
            self.decisions.pop_front().unwrap()
        }
    }
    fn fake(decisions: &[bool]) -> Fake {
        Fake {
            calls: vec![],
            decisions: decisions.iter().copied().collect(),
            fail_at: None,
        }
    }
    const STEPS: &[&str] = &[
        "authenticate",
        "snapshot",
        "grace",
        "observe",
        "observe",
        "authenticate",
        "quit",
        "owner-lock",
        "empty",
        "disable-runtime",
        "disabled",
        "empty",
    ];

    #[test]
    fn removal_reuses_original_fence_and_never_hides_plugin_again() {
        let mut host = fake(&[true, true]);
        assert_eq!(watch_host(&mut host, "one-operation"), Ok(true));
        assert_eq!(host.calls, STEPS);
    }
    #[test]
    fn reload_and_reenable_before_final_admission_do_not_disconnect() {
        for decisions in [&[false][..], &[true, false][..]] {
            let mut host = fake(decisions);
            assert_eq!(watch_host(&mut host, "one-operation"), Ok(false));
            assert!(!host.calls.contains(&"quit"));
        }
    }
    #[test]
    fn stale_busy_and_every_host_failure_stop_without_retry_or_forced_cleanup() {
        for index in [0, 1, 5, 6, 7, 8, 9, 10, 11] {
            let mut host = fake(&[true, true]);
            host.fail_at = Some(index);
            assert!(watch_host(&mut host, "one-operation").is_err());
            assert_eq!(host.calls, &STEPS[..=index]);
        }
    }
    #[test]
    fn fixed_failure_is_public_and_truthful_after_ui_is_hidden() {
        assert!(FAILURE.is_ascii() && FAILURE.len() < 180);
        assert!(!FAILURE.contains("remains enabled"));
        assert!(!FAILURE.contains('/'));
    }
    #[test]
    fn relative_and_traversing_manifest_roots_are_not_removal_evidence() {
        assert_eq!(manifest_at(Path::new("relative"), 0), Manifest::Unknown);
        assert_eq!(manifest_at(Path::new("/../"), 0), Manifest::Unknown);
    }
}
