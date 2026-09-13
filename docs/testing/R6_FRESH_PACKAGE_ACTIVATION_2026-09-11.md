# Fresh packaged activation and primary native controls

Local-only Try Omarchy ARM64 acceptance, 2026-09-11. No push, PR, main merge,
marketplace release or claim of complete R6. Owner requested restoring the
prominent mode controls/profile frame and testing the clean package path.

## Exact artifacts

| Artifact | Identity |
| --- | --- |
| Combined source/frontend | `82a4444880bdeefcc00e64d5d0556488212d9be6` |
| Absent-legacy implementation within it | `d270fd568432ada56ffb7d2348006a6a8feeb147` |
| Installed package | `omavless 0.0.0.r469.g82a4444880bd-1`, aarch64 |
| Installed/package binary SHA256 | `c8a5b4883aeed2672c3f3bfa55d055c3cec7f7acf439b09c2da1a368c6e7d639` |
| Package archive SHA256 | `a7105fbabbe5f49f52b60f9d520f97c2bf4081b90a9416a6947089c438ba90e5` |

The locked local build reused the validated Rust implementation; the subsequent
source change was QML and its regression tests only. The normal Arch helper
verified clean source and identical staged/archive binary bytes. Actual
`pacman -U` enforced runtime dependencies and upgraded the installed package;
no `--nodeps` installation, remote download or service-enabling package hook
was introduced. Package identity was verified after installation.

## Primary interface restoration

The retained Python/reference panel has three directly visible mode choices
above traffic and profiles. The native panel had replaced those with a link
to Settings. The same native action controls now appear prominently on main,
ordered Full VPN / Routing / Direct with equal-width allocation. They retain
existing `nativeCanAct`, current-mode and semantic action fences, are keyboard
targets, and do not set desired/actual state optimistically in QML.

Search and profile/group rows now share a square, transparent bordered surface
with explicit padding. The existing outer Flickable and reserved scrollbar
gutter remain; no nested scroller was added. Grouping, selected-row actions,
search and arrow navigation still use the existing models/handlers. Settings
can still access the same controls; this is not a return to Python dispatch.

The native-only installer deployed the exact candidate. All 19 tracked files
below `plugin/` matched installed bytes. The installed main screen was opened
and inspected through a private local screenshot: three mode controls are
visible above a padded, framed search/profile list; the disconnected header and
compact import/row actions remain aligned. No mode button was activated during
this visual check, so it is not new connection/authentication evidence.
Private screenshots are not committed or shared with this report.

Shell rescan initially exposed a transient missing target; the normal shell
restart reported a readiness timeout while the new shell was still loading.
It subsequently answered ping and plugin IPC and rendered the candidate. No
shell source, authentication policy or VPN process was modified to force it.

Validation: 23 focused native-main tests, full reference suite 411 tests with
four expected skips, all JS/QML contracts, plugin validation and diff check PASS.
The inherited Rust source gate remains 944 passed / ten existing ignored and
55 compiled no-Python CLI cases, as documented in the preceding checkpoint.

## Real fresh-user package path

A password-locked, no-login-shell disposable local user with an empty home was
created explicitly for the test. Its real systemd user manager was started
temporarily through linger. No user's profiles were copied, imported or read.
This is a fresh account on the existing Try Omarchy host, **not a pristine OS
image**: normal host-wide user services may start for a new user. No shared-Mac
folder content was used or modified.

| Gate | Result |
| --- | --- |
| Installed native executable and dependency checks | PASS |
| Legacy service `LoadState=not-found`; native unit disabled | PASS |
| `setup initialize`, two private files, startup Off, no ownership activation | PASS |
| `cutover activate`, committed Rust ownership generation2 | PASS |
| Real runtime observation: Routing, disconnected, no manual recovery, core/TUN0/0 | PASS |
| Semantic snapshot: zero profiles/subscriptions | PASS |
| Legacy unit remains absent; native unit remains disabled | PASS |
| Repeated activation refuses and retains committed Rust owner | PASS |
| Test user manager stopped, linger removed, account and synthetic home deleted | PASS |

The installed package's actual binary and actual packaged user units performed
the activation, not systemctl/socket doubles. The resulting test daemon was
stopped during cleanup. No connection, DNS/route mutation, protocol credentials,
network fixture or TUN setup was requested.

The first test script left its result root-readable only. A second, separately
explained terminal authorization read that existing bounded synthetic report;
it did not repeat installation/activation. The readable transcript confirms
every PASS above and empty stderr for initialization, activation, observation
and snapshot. Duplicate activation produced the expected fixed public refusal.
Future harnesses should capture root-command stdout through the ordinary user's
logger so reading evidence does not need another authorization.

## Final state and remaining gates

The real user's store was not changed. Final read-only observation: Routing,
disconnected, manual recovery false, Mihomo/auxiliary/TUN `0/0/0`. The native
frontend remains available and its installed bytes match the candidate.
The global executable is upgraded; the existing real user's daemon was **not**
restarted/re-executed merely for this test. Actual new-binary daemon execution
was demonstrated by the disposable user's activation, not inferred from the
old user's still-running process.

This closes the missing-legacy **packaged activation** gate for a fresh account
on Try Omarchy ARM64. It does not prove full graphical first-user onboarding,
real login Off/Last/pinned behavior, post-disconnect manager restart, package
removal/rollback/recovery, controlled DNS/auth or the full installed application
with Python unavailable. Python was not masked during this installed host gate;
the separate 55-case masked CLI matrix is not substituted for it. V0 remains
partial, and production TUI remains gated behind complete R6.
