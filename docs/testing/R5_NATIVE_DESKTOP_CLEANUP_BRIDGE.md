# Native frontend startup scratch cleanup

Local frontend code head: `de3b8c88215d31318430cb02078bac4151336684`.
Installed native package remains `0.0.0.r441.g9bf5781c6c2e-1`, built from
`9bf5781c6c2ef20168cb25a25e8a6ae8ec427065`. This follow-up changes only the shell
launcher and its tests: no Rust binary, VPN lifecycle, QML rendering, protocol,
privilege or persistent-state semantics changed. No push or main merge.

## Gap and fixed boundary

`Service.qml` starts `cleanup-runtime` before asynchronous ownership discovery.
Its default `nativeOwner=false` therefore cannot safely choose the implementation.
The canonical ownership-selecting launcher previously rejected this startup call
under committed Rust ownership, despite an existing Rust desktop cleanup helper.

`backend.sh` now accepts exactly one `cleanup-runtime` or `cleanup-qr` argument
under Rust ownership and dispatches only `omavless desktop cleanup`. Extra paths,
flags or shell fragments are rejected without echoing them. Selector/helper
failures preserve failure and never invoke Python. Legacy ownership, including
marketplace-only installs without the native executable, retains the unchanged
Python behavior.

The existing native helper cleans only matching dead-process, user-owned 0600
regular editor seeds in its validated private desktop runtime directory. Native
QR data is in memory; no legacy QR-name sweep or age-based deletion is added.
This is deliberately **not** tunnel cleanup, plugin disable/remove, Quit, service
stop, orphan-core cleanup or an arbitrary-path deletion facility.

## Local gates

- 27 launcher tests pass, including four new dispatch/privacy/legacy/failure
  tests. The real Service startup hook remains covered.
- Full reference command: 349 Python tests, four skips; all invoked JS/QML and
  localization contracts pass. Focused native batch/main tests: 15/20.
- Existing native cleanup filesystem regression: one Rust test passed; other
  tests were filtered, not claimed as rerun. The unchanged runtime's full
  654-test evidence remains at the installed binary head above.
- Shell syntax, compile, manifest, plugin validation and diff check pass.
- Exact frontend installed with `install.sh`; installed launcher byte identity
  verified. Native package was not needlessly rebuilt for this shell-only fix.
- Actual installed launcher ran with a PATH containing only the installed Rust
  executable, with Python unavailable. It removed exactly one deliberately
  created non-secret dead-editor test seed, with no supplied cleanup path.
- Requested state, private store bytes and native daemon PID were unchanged.
  Final Routing/disconnected, zero core/auxiliary/TUN, no manual recovery;
  installed frontend diagnostics remained coherent.

The removed sentinel was synthetic test data only, not a real draft. No real
private editor contents, filenames, profile metadata or helper output beyond
the aggregate removal count were published.

## Remaining distinct migration work

The native launcher still rejects `watch-plugin-removal`. Porting its documented
disable/remove disconnect semantics is a separate lifecycle checkpoint with
reload-grace, ownership/revision, connected cleanup and installation/removal
acceptance. Do not map it to desktop cleanup or an unrestricted shell watcher.

Other bounded audit findings: native headless `importConfig(path)` still rejects,
some public QML IPC read projections remain migration placeholders, and support
configuration reporting is intentionally narrower than full host diagnostics.
Interactive native import, support copy/file export, profile QR/file export and
on-panel routing/details are already implemented and should not be recreated.
R6 Python-absence, package recovery and complete required-operation acceptance
remain separate gates. See `CONTROL_PLANE.md` section 9 for the distinction
between UI-only Quit, disconnect, disable/remove and administrative runtime stop.
