# R6 closure ledger — local candidate, 2026-09-12

This is the current **finite acceptance remainder**, not a release announcement.
Historical source checkpoints remain valid for their recorded heads; an old
"not implemented" paragraph does not override its linked later implementation.
No main publication or production TUI start is authorized by this ledger.

## Candidate and environment

- Try Omarchy, aarch64 VM on Apple Silicon.
- Session-start frontend/source: `66906524d193c2336d2bd43e9e446c102193580e`.
- Refreshed `origin/main`: `27e2e793f0e14f19f41dce947e06667ca9bf5ec3`.
- Local branch: `codex/local-native-probe-execution`; no push/merge in this pass.
- Installed package: `omavless 0.0.0.r479.g9a47a60bbf6d-1`, source
  `9a47a60bbf6d7a5802d2881d1bf25a2988182945`.
- Installed executable SHA-256:
  `c765289019c22aff46e0a2e590c556470fb605361c856bf369479e5dbac15f02`.
- At the original pause the running daemon was still the accepted `46b4413` binary.
  Updating the disk executable/frontend does **not** upgrade that running process.
  That pass preserved the existing Routing connection. On the subsequent resume,
  read-only inspection instead found Routing/disconnected, core/TUN 0/0 and the
  running daemon digest equal to the installed `9a47a60` executable above.

The owner requested automatic work with no password dialogs or manual clicks.
The [host-authorization barrier](HOST_AUTHORIZATION_ACCEPTANCE.md) still applies:
do not script its human acknowledgements or bypass it to turn an unrun host
scenario green. Current connection preservation is not proof of a fresh start.

## Implemented boundaries: do not rewrite these again

| Boundary | Existing evidence / qualification |
| --- | --- |
| Native profile, subscription, routing, startup preferences, diagnostics and desktop-helper dispatch | Restored QML uses fixed Rust launcher commands; [dependency audit](R6_PYTHON_DEPENDENCY_AUDIT.md). Legacy implementation remains a reference/rollback path, not a hidden native fallback. |
| Empty private config preparation and first activation | [Fresh installed account](R6_FRESH_PACKAGE_ACTIVATION_2026-09-11.md) passes real packaged initialize/activate without an old legacy service. Preparation alone is not activation. |
| Native-only installed frontend | [Payload gate](R6_NATIVE_ONLY_FRONTEND.md): installed frontend omits backend.py and legacy remover. `install.sh --native-only` requires already committed ownership; ordinary compatibility install is not a new automatic cutover. |
| Primary/IPC file import and profile export | [Installed UI/IPC](R6_NATIVE_IPC_FILE_IMPORT.md), with confirmation and credential-safe private output. |
| Support report | [Schema-2 composition and owner-confirmed Copy report](R6_NATIVE_SUPPORT_COMPOSITION.md). Explicitly bounded; it does not verify all legacy doctor fields. |
| Login Off without Python | [Actual empty-account user-manager gate](R6_INSTALLED_NO_PYTHON_LOGIN_OFF_2026-09-11.md). Covers a new epoch and same-session disconnected restart, not Last/pinned or connected startup. |
| Full Quit and plugin disable/remove | [Quit](R5_NATIVE_FULL_QUIT.md), [plugin removal](R5_NATIVE_PLUGIN_REMOVAL.md). Package uninstall and the outstanding authorization/Disconnect correction are separate. |
| Latest requested main-panel simplification | Duplicate mode caption removed at `6690652`; the three mode buttons remain. Test and latency sections are intentionally hidden, not missing backend implementations. |

## Required remainder before calling R6 complete

| Gate | What closes it | Current boundary |
| --- | --- | --- |
| Installed Python-unavailable application matrix | Exact installed frontend, CLI **and daemon**, with interpreter execution genuinely unavailable, exercising representative import/profile/subscription/routing/diagnostics and lifecycle paths | [Actual installed domain matrix](R6_INSTALLED_NO_PYTHON_DOMAIN_2026-09-12.md): 12 profile/rule/mode/support stages and real service restart PASS under the mask. Graphical/provider/live-VPN paths remain. Compiled conformance and Off/login are additional subsets, not substitutes. |
| Login Last/pinned | Real fresh user-manager epoch with valid existing fixture; intended mode/profile selected once; explicit Disconnect then same-epoch restart does not reconnect | Saving these preferences and ordinary restart already have evidence. Fresh connected login remains distinct and can invoke host authorization. |
| Disconnect/DNS cleanup correction | One observed transition at a time, all authorization resolved, fresh owned cleanup facts and `manualRecoveryRequired=false` | [Correction](R5_NATIVE_DISCONNECT_PROCESS_EXIT.md); the earlier unattended twenty-cycle result is not complete acceptance. No automatic retry storm or security-policy relaxation. |
| Upgrade, rollback/recovery and package removal | Disposable installed-account/package route; preserve and validate private store/ownership; use supported recovery commands; prove no duplicate owner/core/TUN | Plugin removal is not package removal. Archive inspection is not package-manager recovery. Do not delete markers/receipts to manufacture success or test removal on the owner's connected session. |
| Support contract completion | Implement/test the missing promised host report coverage, or obtain explicit acceptance of the documented narrower report as its replacement | Schema-2 reports truthfully mark unverified DNS/routes/service/login fields. Do not silently promote them or call the narrower report complete legacy doctor parity. |

The Save As UI checkpoint now has owner-confirmed and automated English/Russian
report-write evidence, private-file validation and simple cancellation passes.
The owner confirmed manually saving during the Russian cancel-after-edit attempt,
so that cancellation test is invalidated and needs a repeat, not a product-bug claim;
isolated overwrite confirmation and profile cases still need concrete evidence recorded
in [the export report](R6_SAVE_AS_UI_CHECKPOINT_2026-09-12.md); seeing a
chooser alone is not proof of a successful write. This is a bounded regression gate,
not an invitation to redesign the UI again.

## Automatic checkpoint before the owner-requested pause

Tooling commit `bc94b3c40ba2c6b81e5b5199687438b41a56f5ff` adds a fixed
installed-binary replay to the existing isolated conformance harness, without
changing production code or installed files.

- Installed executable: **56 passed**, five suites (22 CLI, 10 plugin actions,
  7 desktop CLI, 7 owner-target, 10 fresh setup). Python PATH and absolute
  interpreter execution are masked and negatively tested. The reported digest
  matches the package identity above. Synthetic peers/helpers remain synthetic.
- Ten isolation-plan tests, 12 file-export JS cases, 29 native-main cases,
  QML contracts, Python compile, shell syntax, manifest JSON, plugin validation
  and diff check pass.
- Real offscreen Quickshell file-export Process composition passes on retry.
  The first attempt timed out under heavy compilation load. No production
  timeout or safety guard was relaxed.
- The full locked Rust workspace attempt **did not pass**: runtime lib reported
  632 passed / 1 failed / 6 ignored. The failing test is
  `tests::auxiliary_reap_does_not_hold_dispatcher_or_hide_status`, at
  `lib.rs:2987`, where the **post-drain mutation** result had `ok:false`.
  Correction on resume: the original checkpoint misidentified this assertion
  as `status.get`. Status success and its latency assertion at lines 2985–2986
  had already passed. This is not evidence of a hidden/unresponsive status read.
  The focused unchanged test then passed, and the complete sequential runtime
  lib repeat passed **633 / 0 failed / 6 ignored**. This does not establish the
  cause or turn the interrupted workspace/clippy chain into a full green run.
  Next session must characterize the cleanup/mutation failure envelope and distinguish
  fixture timing from a production cleanup defect; do not
  simply enlarge deadlines or suppress the failure.
- Fresh workspace Clippy and subsequent integration groups were not reached
  after that failure. Preserve earlier exact-source evidence, but do not claim
  these checks reran successfully in this pause checkpoint.
- The Save As chooser opened with a prefilled generic report filename. Final
  GUI write/overwrite remains unconfirmed; no test report file was created.
  The dialog was no longer present when the paused-session focus guard ran,
  so no keys were sent to the terminal and no automatic-cancel PASS is claimed.
- One shell-only restart was performed after IPC became unresponsive. Its
  readiness command timed out, but the shell subsequently responded and the
  chooser opened. Original daemon/core PIDs survived; VPN stayed connected in
  Routing, core/TUN 1/1, manual recovery false. No privileged command or host
  authorization was initiated. A temporarily toggled observed-IP display
  preference was restored to its original enabled boolean.

On resume, an unchanged two-thread runtime-lib repeat also passed 633 tests
with six ignored. A sandbox-only attempt separately failed to create the Unix
socket, while the same focused test outside the sandbox passed; that sandbox
failure is not the original cleanup/mutation failure. The mutation assertion
now includes only its stable public error code to make any recurrence diagnosable.
The auxiliary core's bounded stop reserves a TERM grace period followed by
forced cleanup; timing under load is an investigation target, not yet a proven
root cause. No production cleanup deadline or ownership proof is weakened.

The resumed full workspace run (same production source, test assertion diagnostic
only) now passes with `RUST_TEST_THREADS=2`: **952 passed / 0 failed / 10 ignored**
across 72 Cargo result groups, including runtime lib **633 / 0 / 6**. Formatting,
workspace Clippy with warnings denied and the two-case R0 parity smoke also pass.
The reference runner passes **412 tests / 5 skipped**, its JS suites and QML
contracts. This supersedes the earlier uncompleted static run, but does not
explain away its intermittent post-drain mutation failure or replace host gates.
The installed acceptance tool now rejects sticky recovery even when core/TUN
counts are zero; its deterministic policy suite passes ten tests. No runtime,
installed file, network or authorization behavior was changed by these checks.

Raw logs and screenshots stay outside Git in `/tmp/omavless-r6-final-local`.
These temporary artifacts may disappear at reboot; this sanitized summary is
the durable evidence. Generated Cargo cache was cleaned to recover disk space;
source, installed program and the existing package archive were preserved.

## Human-attended installed cycle on resume

Tooling head `7a2da4060a2724a5a30343d87905c3b6821a15e2`; installed package,
disk binary and running daemon identify source `9a47a60` and the digest above.
The owner explicitly resumed availability for host authorization. The checked-in
installed gate ran in a visible normal Omarchy terminal with separate human
`ready`/`settled` acknowledgements, not an agent-fed pseudo-terminal. No repeated
cycle, security-policy alteration or unattended authorization retry occurred.

| Check | Result |
| --- | --- |
| Initial native state | Routing/disconnected, startup Off, manual recovery false, core/TUN 0/0 |
| Full VPN transition | PASS; local command 237 ms (not human authorization duration) |
| Runtime ownership | PASS; one Mihomo child owned by native service, one TUN |
| Controller | PASS; authenticated owned private Unix controller, actual global mode |
| TCP listener classification | PASS; PID/inode attribution permits only expected loopback proxy and system-TUN forwarder, no TCP controller config |
| TUN-bound public HTTPS | FAIL: `probe_timeout`; successful TUN use not demonstrated |
| Disconnect and restoration | PASS; all per-action human barriers completed, disconnected Routing restored, no core/TUN/auxiliary core, manual recovery false |
| Same public HTTPS destination after disconnect | HTTP 200, about 215 ms; not VPN evidence |

The entire installed gate **FAILS** because HTTPS failed. Lifecycle/cleanup and
controller evidence remain useful and distinct. This single attended cycle is
not a repeated churn gate, a proof of DNS/route restoration, or the complete
Python-unavailable installed application matrix. Timeout alone does not identify
DNS, provider interoperability or VM networking as the cause. No production
protocol change or timeout relaxation was made to turn this result green.
Private fixture data was kept in memory; only fixed public results were emitted.
The last read-only observation confirms Routing/disconnected with recovery false
and core/TUN/auxiliary 0/0/0. Runtime remains enabled; no service restart occurred
during this cycle. Nothing was published to GitHub.

## Deferred work that does not expand R6

- The independent DNS/provider/connected subscription-probe investigation
  remains explicit in the main roadmap; no blanket VM, Rust or ISP diagnosis.
- Test/Ping main sections stay hidden until an explicit owner request; follow
  [the restoration switches](../roadmap/MAIN_PANEL_DEFERRED_SECTIONS.md).
- No new protocol fixtures, V0 maturity claim, AWG work or production TUI.
- No x86/bare-metal or NixOS-host claim from this ARM64 Arch VM. The existing
  binary/service contract remains available for the Nix packaging track.

## Publication discipline

Keep the candidate local under the owner's current instruction. Before a later
publication: fetch again; reconcile remote work instead of rebasing blindly;
record the exact runtime/frontend identities and affected gates; preserve
golden/differential corpora and the tested recovery path; review the complete
diff and private-data exclusion. Do not turn a green subset or a disk-installed
binary into an R6-complete claim, and do not push directly to main.
