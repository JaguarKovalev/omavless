# Native-aware installer picker reporting

2026-09-11, local-only; no actual plugin/package installation in this session.

The old installer could find Python GTK4 and suppress its missing-picker notice
even when the committed Rust owner cannot use that fallback. Rust desktop
capabilities deliberately report `gtk4FallbackAvailable=false`. This was an
onboarding/reporting mismatch, not a reason to add Python back to native helpers.

The installer now checks standard zenity/kdialog/yad availability first, without
launching a chooser or invoking Python. If none exists, the canonical fixed
`omavless plugin target` read authorizes the optional GTK probe only for legacy
ownership. Native, unknown, malformed or failed selection never invokes Python
or claims GTK is usable. Without the native executable, the launcher's same
conservative ancestor/ownership-artifact absence policy permits the standalone
legacy marketplace fallback; native or ambiguous artifacts suppress it.

This is dependency reporting, not owner admission or a cutover. No JSON store
repair, ownership/receipt write, privilege grant, package download or automatic
dependency installation is added. Legacy GTK fallback is preserved. The
installer still copies `backend.py` for the compatibility window: native-only
frontend packaging and first-install composition remain separate R6 work.

Arch optional dependency prose now describes yad as an alternative picker,
not an editor. Native profile editing requires zenity; qrencode remains the QR
helper. This documentation correction does not change required dependencies.

Nine deterministic tests execute the real installer in isolated temporary HOME
directories with only ordinary file utilities and harmless Omarchy/native/Python
stubs on PATH. They cover each standard picker, native Python absence, legacy
GTK success/failure, unknown/failed selection, missing executable with regular
or dangling ownership artifacts, unsafe state roots and default state location.
No real shell, service, private profile store, clipboard or auth command is used.

Focused tests, shell syntax and package payload tests pass. Installed native and
legacy onboarding smoke remains pending; these tests do not claim live chooser
or current installed identity acceptance. R6 is not complete.
