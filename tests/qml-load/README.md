# Installed QML compilation gate

Run `bash tests/test-qml-load.sh` inside Try Omarchy, or pass an absolute
candidate `plugin/Panel.qml` path. Requires installed Quickshell, Omarchy
imports and a working Wayland session (exit 77 if absent).

The isolated shell compiles the component graph without creating the plugin.
It does not instantiate Service, access profile state, or change the tunnel.
Wayland is required because Omarchy's PanelWindow type needs its backend even
for compilation; offscreen cannot validate this graph. Each runner has a
private temporary configuration and a 30-second lifetime bound (plus at most
two seconds for forced process cleanup). Raw engine
errors are not published. A positive compile marker is mandatory.

Self-check: `valid.qml` must pass and `invalid.qml` must fail. They cover the
Process/default-property error that textual QML contracts previously missed.
This is not visual acceptance or proof that bindings and actions execute.
No production package files or runtime ownership change.

This gate was reconciled from Draft PR #224 into the local R6 candidate. It is
opt-in, not part of the display-independent `tests/run.sh` suite. Run both
fixtures explicitly before accepting its result:

```sh
bash tests/test-qml-load.sh "$PWD/tests/qml-load/valid.qml"
# Expected exit 1, with the fixed FAIL classification:
bash tests/test-qml-load.sh "$PWD/tests/qml-load/invalid.qml"
bash tests/test-qml-load.sh
```
