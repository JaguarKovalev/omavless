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
