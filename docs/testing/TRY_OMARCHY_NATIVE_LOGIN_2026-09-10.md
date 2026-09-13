# Try Omarchy native startup integration — local acceptance checkpoint

Environment: Try Omarchy ARM64 VM, 2026-09-10. Owner requested local-first
development: no push, PR creation or main merge for this checkpoint. This is
partial R5 acceptance, not completed Rust migration or R6 Python retirement.

## Exact installed candidate

- Source: `5421a3edf3c9e1ec7b8f3caa2f5d59168c6e3867`, local branch
  `codex/scratch-native-tools-ui`.
- Package: `omavless 0.0.0.r428.g5421a3edf3c9-1`, aarch64.
- Installed executable SHA256:
  `d253c8012c586a6523c6736c64c6ca83cd2ed9eb48372f714b74d598a618cddf`.
- 21 runtime-relevant frontend files (QML, JS, backend launcher and manifest)
  match checkout bytes. Exact packaged unit bytes are checked by the login host.
- Prior package `0.0.0.r421.g51ee8204bc4a-1` retained outside Git for rollback.

This combines native UI work, the unavailable-client error fix, isolated
validation and native login integration. It is not a new marketplace release.
The reference Python backend remains in the plugin payload as migration oracle
and rollback material; the tested actions used the Rust binary and native owner.

## Local static gates

- Runtime: **598 passed, 5 ignored, 0 failed**, 14 result groups. Ordinary
  environment-gated tests are not counted as fresh installed-core evidence.
- Python reference: 345 tests, five skipped in the restricted run; all invoked
  JS suites, localization and QML contracts passed.
- New startup UI: seven checks; combined socket regression covers capability
  refusal, canonical plugin dispatch, exact replay, stale instance/revision and
  unchanged desired state/host effects.
- Strict all-target runtime clippy, formatting, shell syntax, manifest JSON,
  diff check and Omarchy plugin validation passed.
- Actual Quickshell import graph compilation passed. An initial offscreen-only
  harness could not load the Wayland PanelWindow backend; this was corrected in
  the harness, not by changing production QML.

## Installed checks

| Case | Result | Observed boundary |
| --- | --- | --- |
| Packaged condition + preparation | PASS | Fixed real systemd invocation accepted; oneshot exited successfully; receipt private and consumed |
| Off startup | PASS | Rule/disconnected, core/TUN 0/0, private profile store unchanged |
| Disabled runtime unit | PASS | Startup editing capability withheld |
| Explicit runtime unit enable + restart | PASS | Startup editing advertised; no VPN autoconnection enabled |
| Save Off / Full VPN preference | PASS | Preference saved, current connection/mode unchanged, same-manager restart disconnected |
| Save enabled / last profile / Full VPN | PASS | Real imported selection validated offline; no connection started; restart remained disconnected |
| Save enabled / selected profile / Routing | PASS | Same validation/restoration boundary; private ID kept out of output |
| Save while connected | PASS | Existing VLESS connection healthy, one owned core/TUN, private controller configuration verified; core PID and desired intent unchanged after saving |
| Disconnect and restoration | PASS | Core/TUN returned to 0/0; original startup policy restored to configured Off/last/Rule |
| Installed UI diagnostic IPC | PASS | Native controls and fresh local facts; no pending/unknown action |

The first installed smoke queried IPC immediately after systemd Type=exec start
and raced socket readiness, while both services had succeeded. The harness now
waits up to ten seconds for IPC; the subsequent check passed. This is not a
claim that systemd start itself proves application readiness.

The read-only host proof also exposed an actual Omarchy constraint: access to
the user manager's `/proc/PID/exe` is denied. Login identity instead uses the
authoritative root-manager fixed unit/User/MainPID/InvocationID facts, canonical
root-owned unit, process UID and actual parent. No sudo, ptrace-policy change or
privileged runtime channel was added to address that constraint.

## Visual and privacy evidence

The exact installed `StartupPrompt.qml` was rendered in a disposable Quickshell
window with synthetic metadata only: English Off, English enabled/last,
Russian enabled/selected and Russian Off. All four captures were inspected for
layout, meaning, wrapping and overlap. No clipping/overlap or unintended English
fallback appeared; the synthetic profile label remained untranslated as intended.
This is component visual evidence, **not** an automated end-to-end navigation or
keyboard pass through the live Settings panel.

Private records, provider names, endpoints, URI/keys and raw error payloads were
not printed or committed. Test scripts parsed private metadata internally and
reported only booleans, public slugs and safe classifications. No real protocol
fixture was added to Git and no missing V0 family was fabricated or accepted.

## Final state and remaining gates

The canonical runtime unit is now explicitly enabled for user login. This is an
intentional native package setup change: it launches the daemon, not a VPN when
startup policy is Off. Legacy startup stays disabled. The login preparation unit
is active/exited (no persistent process); one Rust daemon remains active, VPN is
Rule/disconnected, Mihomo/TUN are 0/0 and the plugin is enabled.

Still required: a real new-user-manager/login pass, full Settings navigation and
keyboard acceptance, installed negative/cache-failure and lifecycle/upgrade/
rollback coverage. Same-manager service restart is not fresh-login evidence.
Other parity work (notably subscription latency execution and the remaining
cleanup/support surfaces) and R6 acceptance are separate unfinished gates.
No main merge or GitHub write was performed after the local-first instruction.
