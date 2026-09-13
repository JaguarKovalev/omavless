# Local Save As and deferred main sections checkpoint

This is a local-only Try Omarchy ARM64 checkpoint, not completion of R6 or
authorization to publish/merge the migration branch.

## Candidate and installation

- Final implementation: `9a47a60bbf6d7a5802d2881d1bf25a2988182945`.
- Package: `omavless 0.0.0.r479.g9a47a60bbf6d-1`, aarch64.
- Installed executable and release build SHA-256:
  `c765289019c22aff46e0a2e590c556470fb605361c856bf369479e5dbac15f02`.
- Frontend installed with `./install.sh --native-only`.
- One explicitly authorized package-installation dialog; no VPN/core restart.
- The user's already-running daemon remains the older accepted process. Do not
  attribute its execution to the newly installed binary. The new CLI/desktop
  helper is invoked by the frontend without a daemon ownership cutover.

## Implemented changes

Report and profile Save As use the isolated Rust desktop helper, fixed public
default filenames, explicit overwrite confirmation and existing secure export
writer. Input is one bounded locale token; private destinations remain local
output/stdin, never reusable credentials in argv. Cancel produces no write or
error. Stale instance/revision/record results cannot enter the exporter.
Missing pickers and an older installed CLI produce actionable bounded messages.

The first QML FileDialog attempt (`83952bb`) failed installed acceptance:
Quickshell aborted in GLib/GIO/GVfs directory monitoring. It is superseded by
`9a47a60`, not a supported alternate implementation. The crash skill guided
stack/resource inspection; exact upstream cause remains unproven. No extracted
core copy was created or published. No crash report/screenshot is shareable by
default. The final frontend contains no in-process FileDialog.

Both Test and Ping/Packet Loss are temporarily hidden at the owner's request.
Their implementation and tests remain. Restoration instructions and explicit
owner-approval requirement are in
[the deferred-sections ledger](../roadmap/MAIN_PANEL_DEFERRED_SECTIONS.md).
Hidden latency monitoring is disabled; ordinary TUN traffic stays visible.

## Evidence

| Gate | Result |
| --- | --- |
| Rust desktop helper unit tests | 18 PASS |
| Actual desktop CLI tests | 7 PASS |
| Full Python suite | 411 run, 407 PASS, 4 existing skips |
| JS suites and QML contracts | PASS; file-export 12, main-panel 29, ping 9 |
| Rust clippy lib + executable, warnings denied | PASS |
| Release build, local package assembly and installed digest | PASS |
| Main Panel qmllint with Omarchy imports | exit 0; 581 existing-style warnings, not warning-free |
| Manifest, shell syntax, plugin validation, diff whitespace | PASS |
| Connected English main: no Test/Ping section, modes and framed profiles retained | visually PASS |
| Installed report Save As: home/folder chooser and generic prefilled filename | visually PASS |
| Real report save, overwrite and cancel completion through final installed UI | NOT YET fully verified |
| Final profile chooser and Russian chooser visual matrix | NOT YET completed |

An initial full test attempt exposed two new launcher aliases missing from the
exhaustive test matrix; both were added. It also timed out while the first
release build competed for VM resources. The serialized retry passed without
weakening deadlines. Synthetic writer tests continue to cover atomic mode0600
output and symlink/unsafe-parent rejection; these are not substituted for the
uncompleted final graphical save matrix.

The already accepted English/Russian UI-polish and copy-report checks preceding
this chooser change remain separate evidence at frontend `89dd07e`; they do not
prove the new save path. Screenshots and local test outputs stay outside Git.

## Runtime and R6 boundary

After package/frontend installation, the original daemon PID829530 and Mihomo
PID837767 remained running. Read-only observation reported connected Routing,
one Mihomo, one TUN, verified owned controller config, no manual recovery.
The explicit instruction to preserve the connection overrides the localization
skill's usual end-of-test disconnect instruction.

Fresh native initialization/activation and the Python-inaccessible empty-user
startup-Off gate already passed in their own reports. Remaining R6 acceptance
includes the complete graphical/workflow run with Python inaccessible, Last/
pinned login, and applicable upgrade/removal/rollback/recovery checks. The
controlled DNS/auth checklist remains explicit, not dismissed as a VM/provider
assumption. UI cosmetics and the two intentionally hidden sections must not
continually expand the migration completion boundary.

## Resumed UI check, 2026-09-12

The installed disk and running daemon now both match the `9a47a60` digest above;
the earlier retained-process qualification describes the original installation,
not this resumed run. An attended installed lifecycle cycle and its failed
TUN-bound HTTPS probe are recorded separately in the
[closure ledger](R6_CLOSURE_LEDGER_2026-09-12.md#human-attended-installed-cycle-on-resume).

English Settings, the focused Save file button and the prefilled report chooser
were inspected again. GUI automation refused to send a synthetic destination
when it could not prove a unique focused chooser. The intended new report in
the private temporary test directory was not observed, so there is still **no
automated write/overwrite/cancel PASS**. A chooser/overwrite prompt on a capture
alone does not establish which interaction saved a file. Russian completion was
not attempted. Do not misreport this automation/focus gap as a proven exporter
defect or bypass the focus guard by sending Enter to an unrelated terminal.

System locale selection was restored, only the panel was closed, and final
inspection found the plugin enabled, Routing/disconnected, one native daemon,
zero Mihomo/TUN/auxiliary core and `manualRecoveryRequired=false`. No zenity
process remained. No private screenshots, reports or credentials were committed.

### Owner-confirmed Save and resulting file

The owner subsequently confirmed pressing Save during that English chooser
session. Read-only verification of the resulting report found a regular,
current-user-owned 1515-byte file with mode `0600`, modified at
2026-09-12 21:15:38 MSK. The installed frontend's `configurationReport` parser
accepted its schema-2 public shape. Contents were neither printed nor committed.
The first verifier invocation used the wrong installed parser location and
therefore produced a generic refusal; retrying with the actual installed
`plugin/NativeSnapshot.js` passed without changing the report.

**English installed report Save As write: PASS (human action + file validation).**
This resolves the write portion of the earlier automation gap. It does not
establish an isolated overwrite-confirmation/cancel test, Russian acceptance or
profile export. Those remain separate from this successful report write.

### Further installed EN/RU checks

Installed package/runtime remains `9a47a60`; all 19 tracked frontend files match
the local candidate byte-for-byte. No product code or installed file changed.
The screenshot/keyboard checks followed the project localization workflow,
with private captures and System locale restored afterward.

The automated focus guard initially assumed zenity owned the visible window.
Actual inspection instead found `xdg-desktop-portal-gtk` presenting zenity's
request. The corrected local test checks one zenity child of the fixed installed
`omavless desktop pick-report-export` command, the actual executable identities,
the expected localized title, a unique matching window and active focus. It
accepts both the launcher's bare `omavless` argv[0] and its absolute path while
requiring the actual executable to remain `/usr/bin/omavless`. No unverified
window receives input; this was an automation correction, not a plugin change.

| Case | Evidence |
| --- | --- |
| English report save to new private temporary destination | PASS: automated write, current-user regular file, 1515 bytes, mode0600, accepted by installed public-schema parser |
| English cancellation | PASS: chooser closed and the intended cancellation target did not exist |
| English overwrite | Atomic replacement of the test report observed (inode changed, valid report and mode0600 retained); isolated confirmation interaction remains unconfirmed |
| Russian report save | PASS: exact Russian request title observed, automated write and same file/schema/privacy checks |
| Russian immediate cancellation | PASS: chooser closed, pre-existing default report's modification time unchanged |
| Russian cancellation after typing a destination | INVALIDATED: the owner confirmed manually pressing Save during this attempt. The resulting file is not evidence of a cancel defect; repeat without concurrent input before claiming cancellation PASS |
| Profile-specific Save As completion | Still pending; focusing its action is not export evidence |

GTK portal-owned labels such as Cancel, Name and OK follow the running host's
English locale, independently of the plugin's Russian choice. The plugin sent
its Russian title and showed Russian surrounding UI; no OS locale or portal
service was restarted to translate system-owned controls. An early capture was
during animation and is not a settled-layout PASS for every chooser element.
The locale-setting command once timed out waiting for shell IPC, but the stored
value, request title and subsequent shell ping proved the requested state; no
restart was needed and no VPN operation occurred.

These results supersede the blanket earlier unconfirmed-write statement, not
the narrower remaining cancellation/overwrite/profile cases. Raw captures and
generated reports stay outside Git; only this redacted matrix is recorded.

Owner clarification after this run: the manual Save action explains the file
created during the intended Russian cancel-after-edit test. This removes that
incident as evidence of a product defect, but does not turn the interrupted
cancellation test into a pass. The successful save and immediate-cancel cases
remain separate evidence.
