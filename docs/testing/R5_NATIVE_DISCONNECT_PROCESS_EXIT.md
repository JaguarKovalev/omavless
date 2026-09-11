# Local native Disconnect: proven process-exit correction

2026-09-11, Try Omarchy ARM64; local-only, no push/PR/main change. This follows
the [sticky recovery finding](R5_NATIVE_PLUGIN_REMOVAL.md#acceptance-tooling-corrections-and-real-follow-up).

## Reproduction and attribution

The strict Mihomo inventory scans every numeric procfs task. Previously every
metadata/open/read/recheck error, including ENOENT or ESRCH after an unrelated
task exits, refused the whole observation. `NativeLifecycleHost` uses this
inventory before transitions and when verifying an empty host afterward.
An observation failure can therefore latch `manual_recovery_required` before
Disconnect, or after it already stopped the core and persisted disconnected
intent. Later zero counts do not clear the coordinator's failure barrier.

On installed source `4d6ddd3168f639448b807afd353f2adf8cfc0b33`, one bounded
sequential harmless-process churn worker accompanied an ordinary Routing
connect/disconnect. The first cycle failed in 139 ms with manual recovery;
desired connected/core1/TUN1 remained. This is a failed baseline, not positive
acceptance. The worker was stopped; the fixed native user unit was explicitly
stopped, and inactive/MainPID0/core0 were observed before package replacement.
No marker, receipt or private intent file was edited to bypass recovery.

Deterministic filesystem phase tests reproduce disappearing tasks throughout
the strict scanner. A real harmless owned child also proves that a retained
`/proc/PID/comm` descriptor returns ESRCH after the child exits and is reaped.
This establishes a concrete inventory defect. The earlier zero-core incident
had no stage evidence, so its exact failing boundary is not retroactively known.
Configured readiness, process-group cleanup and other real errors must still
remain visible; this is not a universal fix for every manual-recovery cause.

## Narrow contract correction

- Initial PID-directory ENOENT means that enumerated task is already absent.
- Later task/comm ENOENT or ESRCH can be skipped **only** after an independent
  PID-directory lookup proves ENOENT. Missing comm while the PID still exists
  remains an error, as do permissions and every other I/O failure.
- Full enumeration, entry/match caps, numeric PID validation, byte bounds,
  symlink/regular-file checks and opened/before/after identity checks remain.
- The inventory root is now revalidated by type/device/inode after the full
  scan, so vanished/replaced procfs cannot masquerade as all tasks exiting.
- No retry loop, longer timeout, process kill, TUN-policy relaxation, controller
  bypass or implicit runtime restart is added.

Eight new regression functions cover phase-by-phase disappearance, injected
ENOENT/ESRCH versus permissions, live missing comm, PID/comm replacement,
missing/replaced root, bounds/later malformed records and real procfs descriptor
behavior. TUN and tolerant legacy-display helpers remain unchanged. Python is
still retained as reference/rollback, not a second active lifecycle owner.

## Safe failure-phase evidence

Disconnect now emits a single fixed stage token on each lifecycle failure:
`read_intent`, `observe_before`, `classify_before`, `write_intent`, `stop_owned`,
`discard_prepared` or `verify_empty`, prefixed by `OmaVLESS disconnect failed:`.
Only compile-time labels reach stderr/user-service journal. There are no
profile names, IDs, endpoints, paths, credentials, raw host errors or new IPC
fields. Original errors and the recovery barrier are unchanged. Rejected
mutations against an already blocked coordinator do not repeatedly re-enter
this path. Success produces no new telemetry.

A regression checks every fixed label and preservation of all original success
and failure results. Logs are diagnostic fallbacks, not translated UI prose.

## Validation

Focused strict inventory suite: 15 PASS, including eight new regressions.
Full workspace: 929 passed, ten ignored; fmt, strict all-target Clippy and the
two-case R0 comparison passed. Python: 367 tests, four skipped; all invoked
JS/QML contracts passed. Compile, shell syntax, manifest, diff check and Omarchy
plugin validate passed. Four explicit installed-Mihomo Rust tests and three
Python installed-core checks passed with Mihomo 1.19.30 linux ARM64.

Exact installed follow-up remains pending at this source checkpoint. Required
installed result is repeated ordinary connect/disconnect
with churn, fresh observed facts and **manualRecoveryRequired=false**, plus
unchanged private-store bytes per complete cycle. Zero counts alone never pass.

This does not close DNS/provider, fresh-login, rollback, Python-absence or all
remaining R5/R6 gates, and does not change V0/#30.
