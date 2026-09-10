# Local native Omarchy disable/remove bridge

Local-only R5/R6 continuation, 2026-09-11. Not a published marketplace change;
no push, PR or merge is authorized by this checkpoint.

## Ownership and scope

Python's `command_watch_plugin_removal` is the legacy reference. Native QML
unload now launches the fixed installed `omavless plugin watch-removal` through
a fixed `/bin/sh` existence check, without depending on any removable plugin
file or completion of QML ownership discovery. A missing optional Rust package
is quiet. Proven legacy ownership is a Rust no-op and retains the legacy
launcher path; unknown ownership never falls back to Python in native dispatch.

Installed Omarchy's disable command changes shell plugin-enabled state. Remove
first unloads enabled QML, immediately deletes/unlinks/moves the checkout, then
rescans. There is no uninstall hook. Source inspected read-only:
`/usr/bin/omarchy-plugin-disable`, `/usr/bin/omarchy-plugin-remove` and
`/usr/share/omarchy/shell/services/PluginRegistry.qml` (locate the packaged
registry if its directory changes). No Omarchy package files are patched.

The watcher owns only fixed host completion, not another VPN state machine:

1. Validate committed native ownership; take a private nonblocking watcher
   lock, separate from `owner.lock`. Simultaneous unloads do not queue retries.
2. Authenticate the installed binary/unit, environment and same-user Unix peer;
   capture daemon instance and revision before a fixed two-second grace.
3. Recheck the bounded shell registry. Enabled always wins. Explicit disabled
   admits cleanup; absent registry entry requires a safely absent manifest too.
   Unknown/malformed/unavailable shell state is **not** removal, even with an
   absent manifest. Symlink, permission, wrong-type and unsafe ancestor checks
   never masquerade as file absence. Registry output is capped at 64 KiB/1024
   rows and duplicate matching rows/id/enabled fields are rejected.
4. Make the final registry observation before opening a new authenticated unary
   stream (the shell query may take longer than the daemon framing deadline).
   Send `runtime.quit` with the **original** instance/revision, without a stale
   retry. Reuse canonical disconnect, auxiliary drain, graceful runtime exit,
   owner lock, strict empty-host proof, fixed native unit disable and recheck.
   Never invoke plugin-disable again: Omarchy already unloaded it.

Full Quit and this watcher share runtime completion. Neither kills arbitrary
processes, deletes root-owned units, clears startup preferences/login receipts,
changes firewall/security policy nor introduces privileged commands. The
existing reconnect/DNS authorization path is unchanged. A failing detached
cleanup produces only a fixed English recovery notification, never raw private
backend output; it does not pretend that an already hidden plugin is enabled.
Russian translation of this new out-of-process fallback is not yet claimed.

## Deliberate limits

- The final disabled/absent observation is the admission point. Omarchy state
  and daemon desired revision are not an atomic cross-process transaction.
  Re-enable observed before admission cancels cleanup; a re-enable afterward
  can race a legitimately admitted disconnect. No race-free shell transaction
  or automatic reconnect is claimed.
- Busy, stale instance/revision, package mismatch, authentication failure or
  unverified cleanup stops the sequence. No refresh-and-retry against successor
  intent, forced process kill, silent re-enable or success claim is allowed.
- Shell SIGKILL cannot run QML destruction. Crash/UI loss remains tunnel-neutral.
  An unavailable shell can leave cleanup unattempted; explicitly re-enable and
  use Quit once the shell is healthy. There is no new persistent watchdog.
- A stopped, disabled unit causes an idempotent no-op after Full Quit; that
  short circuit is not an additional empty-host proof.
- This is **plugin** disable/remove, not native package uninstall. The old
  `uninstall.sh`, including `--purge`, now refuses before any commands when
  ownership artifacts exist or their absence cannot be proved. It must not
  delete Rust-owned state even after daemon shutdown. This admission guard does
  not claim atomic coordination with a separately starting cutover.
- Reopening uses the existing explicit native-unit/plugin enable sequence in
  [Full Quit](R5_NATIVE_FULL_QUIT.md). A single-action reopening UX is separate.

## Deterministic evidence

- Runtime suite: 675 passed, six ignored. Seven watcher tests cover decision
  states, registry ambiguity, old-fence reuse, reload/re-enable no-op, every
  host failure, safe error and invalid paths; one CLI test rejects all extra
  parameters before host access. Existing full-Quit fences/connected exit pass.
- Python: 367 executed, four skipped. Includes 15 new legacy-uninstall guard
  tests and fixed native watcher launcher coverage, with all host effects
  stubbed. The first full run exposed two legacy tests inheriting real
  `XDG_STATE_HOME`; their synthetic environments were corrected, not the guard.
- Seven Quit/QML JS tests include destructor execution before/after owner
  discovery. Full invoked JS/QML contracts pass.

Strict clippy all targets, fmt, compile, shell syntax, manifest, plugin validate
and diff checks pass. Four installed-Mihomo opt-ins also pass (1.19.30 ARM64).
DNS/provider and fresh-login/Python-unavailable acceptance remain independent.

## Installed acceptance

Try Omarchy ARM64, exact Rust/frontend source:
`4d6ddd3168f639448b807afd353f2adf8cfc0b33`.
Installed package `omavless 0.0.0.r450.g4d6ddd3168f6-1` matches its binary build
identity; 25 installed QML/catalog/template/manifest/launcher/uninstall files
match the checkout. Later evidence-only commits do not change these bytes.

| Public case | Result | Runtime/core/TUN outcome | Restoration |
| --- | --- | --- | --- |
| Disconnected plugin disable | PASS | runtime stopped/disabled; core0/TUN0 | enabled, Routing/disconnected |
| Connected plugin disable | PASS | one core/TUN to zero; runtime stopped/disabled | enabled, Routing/disconnected |
| Connected plugin remove | PASS | checkout removed; installed watcher still stops runtime/core/TUN | original checkout recovered, enabled, disconnected |
| Connected shell reload | PASS | same daemon PID, same core PID, one TUN | disconnected, no manual recovery |
| Disable then immediately re-enable | PASS | grace cancels shutdown; same daemon/core, one TUN | disconnected, no manual recovery |

The actual ordinary-directory install was checked to be current-user owned,
non-symlink and not a Git checkout before `omarchy plugin remove --yes`. Omarchy
made its recoverable hidden backup. Only that newly created exact backup was
moved back, followed by the supported rescan/enable flow. No private profile,
subscription, ownership marker, login receipt or package was removed.

All successful shutdown cases independently verify inactive/disabled native
unit, zero daemon/watchers/core/TUN, hidden plugin and preserved private-store
bytes. Final restored state requires fresh non-null facts **and**
`manualRecoveryRequired=false`, not just a disconnected desired flag. No
provider/network reachability is claimed by a local lifecycle test. Re-enable
after final admission, lost-auth recovery, translated notification and fresh
login were not tested and are not implied by this table.

### Acceptance tooling corrections and real follow-up

Initial reload/re-enable wrappers wrongly compared a disconnected store hash
with its connected compatibility-pointer state and compared unsorted systemctl
property strings. Later wrappers compare like-for-like states/individual PID
facts. A further immediate post-connect observation was temporarily unavailable;
the wrapper now waits boundedly for fresh read-only facts, without retrying any
mutation. These premature runs are not successful end-to-end evidence.

Separately, one ordinary restoration Disconnect left desired disconnected and
core0/aux0/TUN0 but **sticky manual recovery**. The early wrapper missed that
flag; subsequent connect and disconnect returned `manual_recovery_required`.
This was a real runtime outcome, not repaired by weakening the acceptance test.
After observed disconnected/empty facts, one explicit native-unit restart used
existing startup reconciliation; it restored actual disconnected and cleared
the in-memory failure. Markers, receipts and private data were untouched.
Subsequent strict connected disable/remove/re-enable/reload cases pass.

Source explains the stickiness: `lifecycle.disconnect` persists disconnected
intent before stop/discard/empty verification, and a later failure blocks the
transaction coordinator for that daemon instance. Later zero counts do not
clear that block. Evidence does not identify the original failing boundary
(core/group stop, controller/staged-file cleanup, final ownership observation).
Those paths predate this watcher, but that does not prove the event unrelated.

- [ ] Add bounded credential-free failure-phase evidence, reproduce the normal
  Disconnect cleanup failure and fix its owning boundary before R6 completion.
  Do not replace cleanup proof with zero counts, lengthen timeouts blindly,
  clear ownership/receipt files, or silently restart on every error.

Final state: plugin enabled, native unit enabled/active, one daemon,
Routing/disconnected, core0/aux0/TUN0, manual recovery false. This checkpoint
closes the tested native plugin disable/remove path, not all R5/R6 acceptance.
