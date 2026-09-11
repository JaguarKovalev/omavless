# R6 Python dependency and remaining-path audit

Local source audit, 2026-09-11, at
`6aaebec1a341b8a34001fc28c9a0030919d9ee7a` on
`codex/local-native-probe-execution`. This report is not installed acceptance,
R6 completion, a release announcement or permission to remove the Python oracle.
No network refresh, private-store inspection, runtime commands, build or live
test was performed for this audit. The owner requested local-only work; remote
publication/merge state is not inferred from this checkout.

Policies read: `AGENTS.md`, `DEVELOPMENT_ROADMAP.md`,
`docs/roadmap/DEVELOPMENT_WORKFLOW.md`, `ACCEPTANCE_ENVIRONMENTS.md`, and
`RUST_MIGRATION.md`. Historical checkpoint wording is interpreted alongside the
later local implementation and evidence, not as proof that an already composed
native operation still needs to be rewritten.

## Conclusion

The inspected committed-native frontend dispatch does not fall back to Python.
Most restored UI operations already reach the Rust executable through the fixed
launcher. Remaining Python files and test interpreters must not be confused with
a second live runtime. Conversely, no-Python subprocess observation for one
successful connection does not satisfy R6.

There are still concrete product-path gaps: fresh native initialization,
installer/helper policy, a few existing headless shell IPC operations, full
support-report coverage or an explicitly accepted replacement contract, and
native package removal/upgrade/recovery acceptance. The complete installed
Python-unavailable matrix and fresh-login gates remain open.

### Subsequent local checkpoints in this session

`58b9856` implements fixed `setup initialize` for create-only private empty
config preparation; [contract/evidence](R6_FRESH_CONFIG_PREPARATION.md).
This is tested compiled Rust, not installed activation. Clean-host absent-legacy
admission and complete onboarding/activation remain open; do not confuse
prepared files with committed ownership or finished first-install support.

`3eb2d73` adds and installs an explicitly guarded native-only frontend, omitting
the installed legacy backend/remover and refusing interpreter fallback if the
legacy payload is absent. See [native-only frontend](R6_NATIVE_ONLY_FRONTEND.md).
Default compatibility installation still exists; this closes an opt-in migrated
host payload boundary, not clean-user initialization or full Python-absence.

Later installed UI/IPC evidence at `2fd0d2b` supersedes the old installation
identity and headless-file-import gap below: see
[native IPC file import](R6_NATIVE_IPC_FILE_IMPORT.md). It includes successful
actual UI file export and a shell-restart requirement observed during deployment.
The earlier zero-auth-helper statement below was incorrect because Linux
truncates process names; see the correction in the
[UI audit](TRY_OMARCHY_NATIVE_UI_AUDIT_2026-09-11.md). No new authorization
was initiated by these UI fixes.

The findings below describe the audited source SHA, not permanent deficiencies.
Later local commits address these bounded parts:

- `ef70bb6`: guarded human authorization and isolated no-Python conformance;
  [procedure](HOST_AUTHORIZATION_ACCEPTANCE.md),
  [executable evidence](R6_NO_PYTHON_CONFORMANCE.md).
- `1e95d06`: four cached public shell IPC projections replace early native
  placeholders; [contract and tests](R6_NATIVE_IPC_READS.md). Headless file
  import and full private/live detail parity are not included.
- `51ebc5e`: native-aware installer reporting no longer probes Python GTK for
  native/unknown ownership; [policy and tests](R6_INSTALLER_PICKER_POLICY.md).
  The compatible legacy frontend payload is still retained.

No installed frontend update or host transition was performed for these
checkpoints. Fresh initialization, native-only distribution, complete support
composition, host/login/recovery and installed Python-absence gates remain open.

Final local gate: 405 Python tests (four expected skipped), all invoked JS/QML
contracts, compile/shell syntax/manifest/plugin validation/diff check PASS.
Focused coverage: 11 launcher, nine installer, 14 cached IPC and eight isolation
plan tests; 45 actual no-Python Rust executable tests and five Rust package/
payload checks PASS. Rust production code did not change in this session, so
the prior 929-test workspace/fmt/Clippy evidence remains applicable rather than
being claimed as rerun. Installed runtime remains source `9de33cd` and frontend
remains source `4d6ddd3`; local IPC/installer changes have not been deployed.
Final read-only observation: Routing/disconnected, one native daemon,
core0/aux0/TUN0, plugin enabled, manual recovery false and zero authentication
helpers. No host authorization command or installed-state mutation was run.

## Dependency classification

| Surface | Source-backed classification |
| --- | --- |
| `backend.sh` with target `rust` | Fixed `native-*` mappings and `status` execute `omavless`. Unmapped commands exit 70; invalid/unknown ownership exits 71. No Python fallback after a native error. |
| Missing native executable | `legacy_without_native()` checks ownership artifacts and their ancestors. Present or unprovable native ownership refuses; it does not invoke Python merely because the executable disappeared. |
| `backend.sh` legacy target | Final `exec python3 .../backend.py` is real legacy runtime behavior, retained for existing marketplace/reference/rollback users. It is not native dispatch. |
| Native `plugin/Service.qml` operations | Native imports, profile/subscription/routing mutations, probes, telemetry, helper operations and Quit use fixed launcher/native commands. Legacy `runControl`, drop/mark-active queues and sampling methods have native guards. |
| Service startup/destruction | `cleanup-runtime` selects Rust desktop cleanup under native ownership. Destruction starts the fixed installed native watcher independently of asynchronous QML owner discovery; the guarded legacy launcher remains only for legacy/undiscovered state. See [removal evidence](R5_NATIVE_PLUGIN_REMOVAL.md). |
| Rust desktop helpers | `desktop_helpers.rs` calls fixed native/desktop tools, not Python. Clipboard uses wl-clipboard, pickers use zenity/kdialog/yad, editing requires zenity, QR uses qrencode. These remain real external dependencies. |
| Rust source Python subprocess matches | Reviewed matches in desktop helpers, route probes, provider refresh, profile transactions, routing presets, store bootstrap, core selectors and resolver tests are inside `#[cfg(test)]` modules/test files. They are parity/test dependencies, not compiled application dispatch. |
| Arch native payload | `stage-payload.sh` installs the native executable, two user units and documentation/licenses; no `backend.py`. `PKGBUILD.local.in` has no Python/Cargo runtime dependency. Existing package tests assert Python absence from the archive. |
| Native user units | Runtime `ExecStart` and login `ExecCondition`/`ExecStart` invoke `/usr/bin/omavless`; no Python. Login intent remains separately fenced to the real user-manager epoch. |
| `install.sh` | Still copies and chmods `backend.py` unconditionally. Without external picker binaries, it also executes an optional Python GTK4 capability probe. This is an installer-path Python invocation, even though probe failure only produces remediation text. |
| `uninstall.sh` | Shell-only legacy remover. Refuses all ownership artifacts or uncertain absence before any commands; it does not implement native uninstall/purge. |
| Developer tests | Python reference/oracle suites, generated synthetic hosts, QML contract checks and packaging harnesses legitimately use Python outside the product path. Preserve their language-neutral corpora before deleting any oracle. |

This is a source audit, not an exhaustive syscall trace of every installed
desktop dependency. Omarchy/system utilities are external host components;
their presence must not be mistaken for an OmaVLESS Python backend dependency.

## Concrete unfinished boundaries

### 1. Fresh native installation is not composed

[`store_bootstrap.rs`](../../crates/omavless-runtime/src/store_bootstrap.rs)
provides the create-only, private empty-store primitive. Its own contract says
there is no CLI/IPC/daemon/UI registration, and its callers at the audited head
are tests only. The installed package does not activate services or create
ownership. Public `cutover activate` is a migration of a compatible existing
legacy store, not first-install initialization. Native login requires committed
ownership and the appropriate receipt.

Required follow-up: a bounded explicit first-install composition reusing those
primitives, with empty/missing state, racing creator, unsafe/corrupt state,
cancelled onboarding and startup-Off cases. Do not create marker/receipt files
by hand or silently run the legacy backend to make the new-user path work.
Already migrated private-store evidence does not prove this route.

### 2. Installer/helper policy needs a native decision

The Python reference can use GTK4 `FileChooserNative`; Rust capabilities
explicitly report `gtk4FallbackAvailable=false`. Native Settings now exposes
missing helper remediation before file import, but the installer's Python GTK
probe can still report a capability that the native helper does not possess.
No runtime Python GTK fallback should be added to Rust to hide that mismatch.

Resolve native-aware installation/reporting and the final frontend payload
policy. An accepted explicit picker dependency/remediation policy may replace
the GTK fallback; otherwise a native QML/portal chooser needs its own gate.
See [desktop helper contract](../roadmap/DESKTOP_HELPERS.md). Package optional
dependency prose also calls yad an editor, while actual native editing requires
zenity; align that wording rather than advertise unimplemented support.

### 3. Headless Omarchy IPC has real remaining placeholders

In [`Panel.qml`](../../plugin/Panel.qml), existing public shell methods differ
from the restored visible native UI:

- `status()` returns `Service.statusText`, whose native branch still says
  read-only/live-health-unavailable regardless of the newer native presentation.
- `routing()` and `details()` explicitly return native-unavailable placeholders.
- `diagnostics()` returns a small migration-state object, not the corresponding
  current native facts/configuration projection.
- `importConfig(path)` still calls `Service.importFile()`, which rejects native
  ownership. It does **not** secretly execute Python. Interactive `importPick`
  and `importPaste` already use native acquisition/preview/confirmation.

Toggle/down and profile-targeted edit/rename/QR/export are already composed;
do not rebuild them because their legacy equivalents retain guards. For
headless file import, reuse bounded `desktop file-read` and canonical preview;
define confirmation/replacement semantics rather than copy the legacy implicit
filename-based replace behavior blindly. Public status/diagnostics must not
expose private endpoint/name/record information from private UI detail reads.

### 4. Diagnostic scope is still deliberately narrower

[`support_diagnostics.rs`](../../crates/omavless-runtime/src/support_diagnostics.rs)
marks live-host/controller/login coverage false. The Settings copy/file export
is a bounded configuration report, not a full legacy support bundle. Live
rule/provider UI diagnostics, core setup facts and native traffic/ping exist
separately. Preserve that distinction and either compose a reviewed safe
support snapshot or explicitly accept a changed product contract before claiming
complete support parity. A literal `doctor` CLI is also not registered at this
head; existing preflight/observation/diagnostic commands are not automatically
that future unified command.

### 5. Lifecycle/package gates are not Python rewrites

Full Quit and direct plugin disable/remove are implemented and have local
installed evidence. Native package uninstall is not implemented by the guarded
legacy script. Package upgrade/rollback, recovery from invalid state and the
documented explicit reopen sequence still need their complete declared gates.
Do not delete packaged units, private data or ownership/receipt state to force
an apparent success. Fresh login must cover Off/Last/pinned preferences and
same-manager restart after explicit Disconnect; saved settings are not proof
of login activation. See [login contract/evidence](R5_NATIVE_LOGIN_INTEGRATION.md)
and [Full Quit](R5_NATIVE_FULL_QUIT.md).

## Feasible next no-sudo slice

These checks can use synthetic temporary state and existing test executables;
none requires the user's profiles, host package installation or VPN transition:

1. Extend launcher regressions with `python`/`python3` failure sentinels and a
   command trace. Exercise every native mapping, missing executable, blocked
   ownership and failed native result; assert no interpreter fallback.
2. Test the real native desktop executable with isolated absolute helper PATHs,
   synthetic file contents and stubbed chooser/clipboard/editor/QR tools.
   Exercise capability/missing-helper/cancel/oversize/symlink cases without a
   graphical display or actual private clipboard.
3. Test the installer in an isolated HOME with all Omarchy/system commands
   stubbed. Capture the current GTK-probe invocation/misreport as a regression
   before implementing native-aware dependency reporting. Never execute the
   real installer for this test slice.
4. Close the small public IPC projection gaps with strict synthetic JS/QML
   contracts: connected/disconnected/unknown/pending/manual-recovery states,
   bounded safe output, stale samples and no provider data. Treat file import
   as its own explicit semantic/confirmation change.
5. Inspect an existing inert package archive for exact payload/dependencies and
   preserve source identity; archive inspection is not installation acceptance.

A Python-written test driver outside the tested application boundary is not a
product runtime dependency. Conversely, PATH sentinels alone do not exclude an
absolute interpreter or shebang: complete installed R6 needs a documented,
application-scoped absence mechanism plus execution evidence, without removing
system Python or weakening host security.

## Required final acceptance still open

### Subsequent isolated conformance evidence

During the same local session, the root agent reported **45 real compiled Rust
CLI tests PASS** under `tools/run-native-no-python.js` and
`tools/native-no-python-entry.js`: CLI 22, plugin action CLI 10, desktop CLI 6,
plugin target 7. The bwrap fixture masks Python executables and isolates the
home/runtime paths; there is no host bus, graphical session, TUN or external
network. This is synthetic executable conformance, **not installed R6
acceptance**. It does not close fresh install, host authorization, real login,
network/DNS or complete graphical parity gates.

- [ ] Complete native fresh-install and existing-store migration routes without
  Python setup, preserving strict validation and user data.
- [ ] Resolve the remaining supported UI/IPC/helper/reporting boundaries above,
  then exercise normal installed status/import/profile/subscription/routing/
  diagnostics/startup/lifecycle with Python deliberately unavailable.
- [ ] Record exact binary/package/frontend identities and no normal
  `backend.py` owner, venv/pip installation or hidden interpreter subprocess.
- [ ] Run one observed host transition at a time. Resolve **all** authentication
  dialogs before another mutation; CLI success and zero core/TUN counts do not
  prove DNS/route authorization settled.
- [ ] Repeat the affected Disconnect correction gate with fresh facts and
  `manualRecoveryRequired=false`. The reported automatic twenty-cycle run is
  explicitly not full acceptance because of unresolved authentication helpers;
  see [correction and interruption](R5_NATIVE_DISCONNECT_PROCESS_EXIT.md).
- [ ] Finish fresh-login and package upgrade/removal/rollback/recovery gates,
  without editing receipts or ownership markers to bypass failures.
- [ ] Preserve the open DNS/provider/independent-network checklist in
  `DEVELOPMENT_ROADMAP.md` and [probe evidence](R5_NATIVE_SUBSCRIPTION_PROBES.md).
  Its deferral is not a claim of a universal VM or Rust DNS bug.
- [ ] Retain usable golden/differential corpora and explicitly classify the
  remaining Python source as legacy rollback or developer-only before removal.

R6 and production TUI remain gated. V0/#30 evidence and unavailable protocol
fixtures are unchanged; this audit does not require new protocol credentials.
