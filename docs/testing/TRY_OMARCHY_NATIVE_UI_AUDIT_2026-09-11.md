# Installed native UI walkthrough — 2026-09-11

Status: local functional/visual audit; **not full acceptance and not R6 completion**.
No deployment, runtime implementation change, push or merge was performed.

## Exact tested identity

- Environment: Try Omarchy ARM64 VM; 3024×1900 physical display, scale 2,
  1512×950 logical desktop. English/System locale only.
- Fetched `origin/main`: `27e2e793f0e14f19f41dce947e06667ca9bf5ec3`.
- Local source at audit start: `ae3b23af9888ce0d4c8432503d8a8269f612d0fc`.
- Installed frontend: `4d6ddd3168f639448b807afd353f2adf8cfc0b33`.
  Byte comparison of 22 tracked files selected from `plugin/`, `backend.sh`,
  `backend.py` and `manifest.json` found zero mismatches. This is **not**
  installation evidence for newer local changes.
- Installed Rust source: `9de33cd0243261e6ce20d352135ea0b77919f56e`.
- Installed binary SHA-256:
  `1713849692563ae0ecaa67dc1e8e27ea40d9c054e38c5c1f573e44095fbd2b1d`.
- Installed Mihomo: 1.19.30, not replaced during this audit.

## Method and boundaries

The actual installed plugin was opened through shell IPC, then exercised with
a temporary same-user Wayland virtual pointer and Hyprland key dispatches.
Rendered screenshots were inspected locally. No OS configuration, permissions,
packages or installed QML files were modified. This does not depend on source
contract assertions pretending to be rendered evidence.

Screenshots and temporary fixture copies were kept outside Git. Raw captures
include private profile details, editor contents and a complete QR credential:
**they are private evidence, not attachments suitable for a PR or public report**.
Only this sanitized ledger is checked in. No private identifiers or credentials
are included here.

There are 69 local captures, including setup/failed-automation captures that do
not constitute passing evidence. The capture directory is mode `0700`; every
PNG is mode `0600`. Two task-generated credential-bearing fixture copies used
for the import checks were removed after the walkthrough; their original
profiles/subscriptions remain in the private store. The clipboard is empty at
the final check, not claimed to preserve its pre-audit contents.

Except for one accidental toggle described below, the walkthrough was performed
disconnected. Existing profiles/subscriptions were not renamed, replaced,
deleted or duplicated. Draft startup settings were canceled, not saved.

## Observed coverage

| Path | Observed result | Limit |
|---|---|---|
| Main header, row actions, tooltips | Rendered, accessible by pointer | Not a complete focus-order audit |
| Search | Synthetic no-match text gives explicit empty result; clearing restores rows | Escape closed the panel in this injected-input test; do not claim field-only Escape behavior |
| Close/reopen with search | Filter remains visible and matches the displayed empty state | Not a hidden-filter regression |
| Settings scrolling | Top, middle and bottom reachable; buttons do not overlap the gutter | Cross-page scroll defect below |
| Routing tools | Opens; child scroll moves while Settings background stays fixed; Close reachable after scrolling | Current disconnected check reports capability unavailable; no live route claim |
| Diagnostics | Opens and explicitly reports unavailable while disconnected | No loaded-rule/core/network evidence |
| Keyboard in diagnostics | Tab focused Back; Enter returned to Settings | Shift+Tab and all global shortcut interactions not certified |
| Onboarding | All three pages rendered; Continue/Back/Escape exercised | Finish, preset changes and connection not committed |
| Startup settings | Off form and unsaved On/last-profile/mode draft rendered | Cancel returned to saved Off; no fresh-login acceptance |
| Subscriptions | List, selected-subscription detail, Edit and Delete confirmation opened | No remote refresh, batch test or destructive confirmation |
| Add subscription | Empty/invalid URL cannot be submitted | No new subscription fetch or persistence |
| Edit subscription | Private URL masked by default; Cancel works | Save not exercised |
| Delete subscription/profile | Explicit confirmation; Cancel works | No deletion performed |
| Rename profile | Centered window rendered on repeated controlled capture | Initial missing-window captures were inconclusive, not a confirmed bug; rename not committed |
| Profile details | Expands saved private metadata with explicit privacy/non-health notice | Private screenshot must not be shared |
| QR | Centered code and credential warning rendered; Close works | No external scanner round-trip |
| Profile editor | Real Zenity editor opened; Cancel returned without saving | No edit/replacement commit |
| File picker | Real GTK portal chooser opened | Existing picker dependency already installed |
| Profile file import | Existing profile export selected through chooser → profile preview → Cancel | No duplicate imported |
| Profile clipboard import | Real profile text → profile preview → Cancel | Temporary clipboard owner used; no import committed |
| Subscription clipboard import | Existing private URL → explicit already-added/no-change result | Does not prove new-subscription confirmation/fetch |
| Subscription file import | File with existing private URL → same duplicate/no-change result | No path-as-URL regression observed |
| Profile export UI | **FAIL**, public failure message, no destination file | Rust-only differential checks below pass |
| Quit | Confirmation explains disconnect/runtime stop/startup disable/plugin disable; Cancel works | Full Quit was not executed |

## Confirmed findings

### Cross-page scroll offset survives reopening

1. Open Settings and scroll to its bottom.
2. Close the panel and reopen it.
3. The main page opens partway down the profile list, with its header/search
   above the visible viewport.

This was reproduced twice. The installed `Panel.qml` open handler resets
`panelFlick.contentY`, while the native page uses `nativeFlick`. Explicit native
Settings/Back navigation resets the native flickable, but fresh panel opening
does not. This is a source-backed explanation of the rendered finding, not a
patch applied during this audit.

### Profile export fails through the UI

The absolute-path confirmation was reached and submitted for new files in
current-user private directories. The installed UI displayed the fixed public
export-failed message; no target was created.

Read-only/isolated checks using the **same installed binary and selected profile**:

- `profile export ... file` succeeds, returns `format: uri` and the expected
  revision, without printing the private response;
- the current frontend `parseQrExport` accepts that exact response;
- `desktop export-file`, given the private path/content through stdin, succeeds;
- its output is a same-user regular file, mode `0600`, with exact content.

Thus the primitive Rust export/read/write path works, while the installed UI
composition does not. The exact QML completion/fence/transport cause still needs
isolation. Do not disable revision/ownership checks to make this test pass.
The safe UI error remained visible during later unrelated successful actions;
error dismissal/lifetime is a secondary UX follow-up.

## Automation and authorization caveats

An initial search attempt used a stale coordinate after the cross-page scroll
problem. The field was not focused and a synthetic `t` reached the panel's VPN
toggle shortcut. This was an **automation error**, not a successful planned
network gate. One explicit UI Disconnect restored Routing/disconnected with
zero core/auxiliary/TUN and no manual-recovery state. No reconnect loop followed.
Network/DNS/authorization acceptance is not inferred from that episode.

Subsequent text entry required a rendered field check. Clipboard automation
initially had a test-owner lifetime/pipe problem; that failed setup is not an
application clipboard failure. The controlled repeat kept the foreground
clipboard owner alive until the preview, then removed the temporary selection.
The original clipboard restoration was attempted but was not durably verified;
the final controlled tests started with an empty clipboard. Do not label this
audit clipboard-state-preserving.

The first process census incorrectly matched the full helper name against
Linux's truncated `comm` field. Corrected `comm` prefix/state/start-time inspection
found **21 live `polkit-agent-he` processes**, all started at 09:59–10:01, before
this walkthrough began around 10:42. They are not zombies. No newer helper was
observed and no password window was visible. These older processes were not
killed or reconfigured. Earlier claims of zero helpers are superseded by this
explicit correction; auth cleanup remains a separate host investigation.

## Final state and remaining gates

- Routing, disconnected, manual recovery false.
- Native runtime remains enabled/running; Mihomo/auxiliary core/TUN = 0/0/0.
- Plugin enabled; inventory remains 22 profiles and one subscription.
- Panel returned to the top of the main page; no editor/chooser left open.
- No application implementation changes, deployment, package changes or GitHub
  writes. No private fixture/screenshot is committed.

Not exercised: Russian or other locale/layout variants, small-display matrix,
all keyboard/focus states, persistent profile/rule mutations, new subscriptions,
remote refresh/test batches, connected diagnostics, HTTPS/DNS/route protection,
mode changes requiring authorization, actual Quit/disable/remove, fresh login,
and full installed Python-absence acceptance. Source/static tests from the prior
session are not substituted for those gates.

Next bounded work: fix and regress cross-page native scroll reset; isolate and
fix UI export completion; repeat the affected installed paths. Preserve the
authorization pause and separate host DNS/login/R6 gates.
# Local fix continuation — 2026-09-11

Candidate `81622be406c345a7f35063c601d9b86358208d03` is installed locally;
all 22 tracked frontend/launcher identity files match byte-for-byte. Installed
Rust binary remains the previously recorded `9de33cd` build. No remote writes,
VPN transitions or authorization requests were initiated in this continuation.

- Native reopen scroll reset: fixed and visually verified by scrolling Settings,
  closing and reopening the main panel. Header and search return to the top.
  The executable handler regression preserves the existing search text.
- Export root cause: `Component.createObject` converts its initial property map
  through QVariant, copying the JS context object. The exact-object admission
  fence therefore rejects the otherwise successful export read. Assigning the
  context after construction preserves identity for profile/report readers and
  the writer without weakening revision, instance or admission checks.
- `node tests/test-native-file-export-qml.js` reproduces failure before the fix
  and succeeds afterward using the actual extracted Quickshell Process component
  and service functions, synthetic reader response and stdin-checking writer.
  It is opt-in and requires installed Quickshell; it accesses no private store,
  socket or VPN. Existing Node export tests now model initial-property copying.
- Local validation: 405 Python tests passed (4 expected skips), all JS/QML
  contracts passed, including 21 main-panel and 9 export checks; the real-QML
  export test also passed. Compile, shell syntax, manifest, plugin validation
  and diff checks passed.
- **Remaining UI gate:** an actual installed profile export through destination
  confirmation and private 0600 file verification must still be repeated. This
  short continuation did not obtain a stable visible destination dialog after
  the export click; no successful UI export or file roundtrip is claimed.

Final observation: Routing, disconnected, no manual recovery, Mihomo/auxiliary/
TUN counts 0/0/0; plugin enabled. The prior auth-helper caveat remains in force.
Raw captures stay private outside Git. These changes are local-only by owner
instruction; this report does not declare full migration/UI acceptance.
