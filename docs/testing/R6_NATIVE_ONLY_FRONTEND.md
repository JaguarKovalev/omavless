# Native-only frontend installation checkpoint

Local R6 packaging slice, 2026-09-11. This is an opt-in frontend for an
**already committed Rust owner**, not first-install initialization or R6
completion. Default `./install.sh` retains the marketplace/legacy-compatible
payload. Existing Python source and differential oracles remain in Git.

## Explicit installation

From the reviewed checkout, use `./install.sh --native-only`. It checks exact
successful `omavless plugin target` selection of `rust` before staging and
again before replacing the installed tree. Missing executable, legacy,
unknown, malformed or failed selection refuses with bounded fixed text.
No ownership marker, receipt, private store, service or host setting is changed.
This is admission checking, not an ownership transaction; later launchers still
perform their own canonical check, so revocation after installation fails closed.

The existing staged replacement preserves the plugin's enabled setting and bar
position. Native-only payload omits `backend.py` and the legacy `uninstall.sh`.
The shared QML/JS presentation and fixed shell launcher remain; native calls
still reach the installed Rust executable. It neither uninstalls system Python
nor downloads/builds the runtime or optional desktop helpers.

If the legacy backend file is absent, unreadable, a symlink or not a regular
file, the launcher refuses before attempting Python, including when canonical
ownership becomes legacy or the native executable disappears. It does not use
a missing file to infer ownership. Default legacy installations retain their
normal regular-file backend dispatch. Native-only missing-picker reporting
never attempts the optional Python GTK probe, even after ownership revocation.

**Further local updates to a native-only installation must use
`./install.sh --native-only`.** Plain `./install.sh` deliberately restores the
compatibility payload; it does not roll back runtime ownership. Native package
removal and actual ownership rollback still require their separate lifecycle
contract; the old shell remover is not an implementation of either.

The earlier installed IPC test found that file replacement/rescan can leave
old QML behavior loaded. Verify running semantics as well as file identity;
use the supported shell restart only when necessary and safe. This installer
does not add an unconditional restart or any connection transition.

## Deterministic acceptance

Tests execute the real installer and launcher under isolated HOME/PATH with
ordinary file utilities and harmless native/Omarchy/interpreter stubs. New
cases cover omission of old payload on update, interpreter absence, missing/
legacy/unknown/failed selection, revocation during staging with intact old
files and cleaned staging, rejected arguments before effects, and no attempted
Python fallback for absent/symlink/directory legacy payloads. No host systemd,
private store, network, chooser or authorization service is reachable.

The installed native payload is a prerequisite for the full application-scoped
Python-unavailable matrix, not a substitute for it. Clean-user initialization,
complete support composition, login, package upgrade/removal/rollback/recovery
and the controlled DNS/host authorization gates remain open.

## Installed local evidence

Exact candidate `3eb2d73d1a4a4c5c5b3597534128245acc954767` installed with the
native-only option on Try Omarchy ARM64. All 21 tracked runtime frontend/
launcher/manifest files match byte-for-byte. `backend.py` and `uninstall.sh`
are absent from the installed plugin; their source remains recoverable in Git.
No system Python files or private profile data were removed. Rust executable
remains the previously recorded `9de33cd` build.

After asynchronous shell reload settled, cached status returned disconnected,
and actual IPC file import displayed the synthetic profile confirmation in the
installed UI. It was canceled without saving. This update did not need another
shell restart; initial IPC requests during reload were unavailable, so those
attempts are not counted as acceptance. No VPN/auth operation was initiated.

Full local tests: 411 Python tests, 4 expected skips, all JS/QML contracts PASS.
The actual Quickshell synthetic export regression also passes. Compile, shell
syntax, JSON, plugin validation and diff checks pass. No Rust production code
changed or Rust workspace test rerun is claimed.

Final state: runtime service active, Routing/disconnected, no manual recovery,
Mihomo/auxiliary/TUN 0/0/0, plugin enabled, 22 profiles and one subscription.
Private screenshots stay outside Git. Local-only; no GitHub/main changes.
