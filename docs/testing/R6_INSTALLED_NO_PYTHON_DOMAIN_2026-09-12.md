# Installed native domain workflows without Python

Try Omarchy ARM64/virtualized, 2026-09-12. **PASS for the bounded installed
domain/CLI/service matrix; not full R6 or graphical/live VPN acceptance.**

## Exact identities

- Test-tool head: `65710ff`, `tests/installed_native_domain.sh`.
- Installed package: `omavless 0.0.0.r479.g9a47a60bbf6d-1`.
- Runtime source: `9a47a60bbf6d7a5802d2881d1bf25a2988182945`.
- Installed and executed daemon SHA-256:
  `c765289019c22aff46e0a2e590c556470fb605361c856bf369479e5dbac15f02`.

No production source, package or main-user service was changed. Native CLI
requests reached the actual installed daemon through its real private socket;
there was no mock executable, controller, socket response or systemctl.

## Host isolation and authorization

A visible Omarchy terminal obtained one sudo authorization for a reviewed
disposable-account wrapper, derived from the accepted
[empty-account Python-absence gate](R6_INSTALLED_NO_PYTHON_LOGIN_OFF_2026-09-11.md).
It created only the locked `omavless-r6-domain` account with an empty home and
minimal user target. A root-owned temporary drop-in scoped to that account's
`user@UID.service` made `/usr/bin/python3.14` inaccessible. Both ordinary aliases
resolve to this interpreter; PATH and absolute negative controls passed.

CLI processes entered the manager's mount namespace before dropping to the
test UID. The actual installed daemon inherited that namespace, checked again
after restart, and its running executable digest matched the installed package.
The main account's interpreter and the OS package stayed untouched.

The checked-in child script requires the exact disposable account/home/runtime
directory, non-root execution, explicit `--run`, no home override and an empty
native baseline. It is not safe or intended to run against the user's store.
Only fixed non-connecting actions are allowed; CLI time and output sizes are
bounded. Synthetic VLESS reserved-address input is **not** a live fixture or
protocol interoperability claim. Subscription preview validates an `.invalid`
URL without fetching it. No real keys, subscriptions or profiles were accessed.
No VPN, route/DNS action, external fetch or graphical helper was requested;
there were no subsequent hidden authorization steps or retries.

## Observed matrix

| Installed operation | Result |
| --- | --- |
| Native empty baseline, startup Off, no recovery/core/TUN | PASS |
| Profile import preview | PASS |
| Subscription URL preview, no fetch | PASS |
| Confirmed profile import | PASS |
| Rename and favorite | PASS |
| Editor-input read and explicit URI file-export read | PASS |
| Confirmed profile replacement | PASS |
| Custom rule add, read and delete | PASS |
| Full/Direct/Routing desired modes while disconnected | PASS |
| Onboarding completion and schema-2 support read | PASS |
| Profile deletion, inventory returns to empty | PASS |
| Final disconnected fresh facts, no recovery, store mode0600 | PASS |
| Native service restart, store digest unchanged, Python still inaccessible | PASS |
| Disposable account/home/namespace-drop-in cleanup | PASS |

All 12 child stages passed. Five deterministic policy tests cover syntax,
effect-free default refusal, wrong-home refusal, guard ordering, absence of
privileged/network commands and explicit fresh empty-host checks. Full reference
runner: **417 tests, four expected skips**, all invoked JS/QML contracts PASS.
Shell syntax and diff check PASS. Test tooling only; the preceding full Rust
workspace's production-source evidence remains applicable.

Raw synthetic outputs and wrapper are local under the private session
`domain-no-python` build-artifact directory, not Git. Public output contains only
fixed stage labels and pass/fail. Cleanup removed the account/home created by
this run and its own unchanged mask; it did not delete application/user data.

## Remaining boundary

This extends actual installed Python-unavailable acceptance beyond an empty
account. It does **not** prove QML/clipboard/chooser execution under the mask,
real provider refresh, live connect/disconnect, Last/pinned fresh login,
package upgrade/removal/rollback or complete support-report parity. Disconnected
mode changes are not live mode transitions. Editor/export reads are not GUI
editor saves or filesystem export writes. No Python source was deleted and R6
remains open; see the [closure ledger](R6_CLOSURE_LEDGER_2026-09-12.md).
