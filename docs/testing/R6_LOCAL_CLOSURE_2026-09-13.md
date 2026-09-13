# R6 native runtime retirement — local closure

Status: **CLOSED for the local native candidate under the owner's revised scope,
2026-09-13**. Not merged, published or a new marketplace release.

## Explicit scope decision

The owner accepted closing the Rust/Python runtime migration while deferring
optional enabled Last/pinned fresh-login acceptance to
[AUTO-1](../roadmap/LOGIN_AUTOCONNECT_FOLLOWUP.md). It remains OPEN, not PASS.
Startup-Off and normal native application/Python-absence gates are retained.
The already-deferred DNS/provider investigation remains separate and unresolved.
No failing probe is converted to success.

This is retirement of Python from the **native installed product path**, not
deletion of every Python source/test or migration of the currently published
marketplace snapshot. The explicit `--native-only` frontend and native Arch
package use Rust. The compatibility installer/reference code remains in Git,
and plain `install.sh` still installs that compatibility payload. Publication
must identify the native installation route; it cannot claim that merging docs
automatically converts existing marketplace installations.

## Exact identities

- Remote main at closure: `27e2e793f0e14f19f41dce947e06667ca9bf5ec3`.
- Local branch: `codex/local-native-probe-execution`.
- Source before this docs-only closure: `4c46395` (UI review skill/contract).
- Installed frontend implementation: `13717a75264aa2be350bdf741d57b4f1210bcbad`;
  later changes are documentation only. All 20 tracked plugin/launcher runtime
  files matched the installed payload in the UI acceptance pass.
- Installed/running Rust source: `7b75b747883d66a05f1f2b321d42f194c6040b5c`.
- Package: `omavless 0.0.0.r492.g7b75b747883d-1`, aarch64.
- Executable SHA-256:
  `fe04fba32d4e135d56e929350cd0296d3f89168e06d6ebba9a6de089aec4b786`.

No Rust/package production changes occurred between that runtime source and
this closure. Evidence below retains its actual test head and limits; it is not
a claim that every historical test reran on the latest docs commit.

## Migration evidence reconciliation

| Requirement | Evidence / outcome |
| --- | --- |
| Native operations, IPC and no hidden Python fallback | [Dependency audit and subsequent corrections](R6_PYTHON_DEPENDENCY_AUDIT.md), [native-only payload](R6_NATIVE_ONLY_FRONTEND.md), [installed UI/IPC import/export](R6_NATIVE_IPC_FILE_IMPORT.md) |
| Deliberate installed Python absence | [Installed bridge](R6_INSTALLED_NO_PYTHON_BRIDGE_2026-09-12.md): 12 domain and 16 actual QML/HTTP stages; [conformance](R6_NO_PYTHON_CONFORMANCE.md) and closure ledger preserve isolated executable results separately |
| Installed connect/Disconnect/cleanup without Python | [Attended cycle](R6_ATTENDED_PYTHON_ABSENCE.md): Full VPN, one owned core/TUN, Unix controller, interpreter restoration and Routing cleanup PASS; HTTPS timeout remains FAIL |
| Fresh empty native installation, existing-store transition | [Fresh package](R6_FRESH_PACKAGE_ACTIVATION_2026-09-11.md) plus recorded native ownership/installed store evidence; no manually forged markers or Python fallback |
| Default startup Off, fresh manager without Python | [Installed Off gate](R6_INSTALLED_NO_PYTHON_LOGIN_OFF_2026-09-11.md), including disconnected same-manager restart. Relevant login/startup implementation and Arch packaging paths are unchanged through current runtime source |
| Compatible package downgrade/re-upgrade/removal/recovery | [Attended package recovery](R6_INSTALLED_PACKAGE_RECOVERY_2026-09-12.md), including recovery to current archive. This is not an ownership rollback to Python |
| Full Quit / plugin disable/remove | [Quit](R5_NATIVE_FULL_QUIT.md) and [removal](R5_NATIVE_PLUGIN_REMOVAL.md), subject to their recorded limits |
| Support/report and frontend parity | [Schema-3 support](R6_NATIVE_SUPPORT_COMPOSITION.md), [Save As](R6_SAVE_AS_UI_CHECKPOINT_2026-09-12.md), [corrected main UI](R6_PROFILE_SELECTION_UX_2026-09-13.md); owner accepted corrected appearance, not every conceivable state |
| Preserved regressions / no new runtime Python setup | Golden/differential corpora remain; previous runtime workspace 957 passed / 0 failed / 10 ignored, fmt/Clippy; 460 reference tests / 4 skipped; latest affected JS/QML suites pass, main-panel 37 tests. Full historical limits remain in the [ledger](R6_CLOSURE_LEDGER_2026-09-12.md) |

### Final preference restoration

The native `plugin startup-configure` action saved Off / last / Routing using
current instance/revision fencing and bounded stdin. This is a future-login
store-only operation, not a systemd/network mutation. Readback proved Off and
unchanged desired state, runtime PID and child PID list. Existing connection
remained Routing with one core, one TUN, zero auxiliary cores and no manual
recovery flag. No password prompt, reconnect, disconnect or shell restart was
needed. The new-user default was already Off; production code was not changed.

Closure-session checks: all 20 installed plugin/launcher runtime files still
match; installed legacy backend/remover are absent; disk and running Rust binary
digests agree. Seven startup UI checks and 11 native launcher/no-Python tests
pass. All 147 local documentation links resolve and diff checks pass. These
focused checks supplement, not relabel or rerun, the earlier full suites.

## Explicitly not closed

- AUTO-1 enabled Last/pinned fresh-login acceptance and its connected-session
  restart/authorization cases.
- DNS/route-restoration correctness and the intermittent provider/path failure.
  A later TUN HTTPS success for a different active fixture is not a successful
  rerun of the failed Python-masked probe or the original reboot.
- Additional Save As cancel/overwrite experiments the owner deferred; hidden
  Test/latency sections remain intentionally hidden, not missing Rust operations.
- Independent bare-metal/x86 and NixOS acceptance, V0 unavailable-family maturity.
- Whole-branch remote reconciliation, final publication review and owner approval
  for main. No push/merge occurred. Historical 0.7.0 remains immutable.

Local R6 closure removes the migration prerequisite for a separately scoped T2
task; it does not start a TUI implementation in this task or waive runtime,
security, host or publication gates. Future evidence must not erase the failed
probe or promote AUTO-1 merely because R6 is marked closed.
