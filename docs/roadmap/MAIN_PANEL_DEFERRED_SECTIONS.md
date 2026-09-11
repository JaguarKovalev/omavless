# Temporarily hidden main-panel diagnostics

Owner direction, 2026-09-11: hide these two native main-panel sections until
their presentation is improved. This is not removal or a completed redesign.
Do not restore them as part of routine parity/cleanup without explicit owner
instruction.

| Section | Gate in `plugin/Panel.qml` | Preserved implementation |
| --- | --- | --- |
| Connection Test / HTTPS result | `showMainConnectionTest: false` | `nativeTestButton`, result/scope UI, Service connection test and fixed Rust probe |
| Ping / Packet Loss | `showMainLatencySection: false` | `nativePingTest`, latency/loss/status UI, Service rolling samples and fixed Rust ping |

The latency gate also hides the adjacent `traffic.native_note` explanation at
the owner's request. Traffic rates/totals and sparkline remain visible; their
scope remains TUN-interface traffic, including both direct and VPN paths, not
a count of exclusively proxied traffic. Do not change that meaning later.

Both gates remove their controls from keyboard navigation. The latency gate
also disables main-panel background ping monitoring, so hiding the section
does not leave invisible probes running. Backend/CLI functionality, translations
and deterministic tests remain intact. No connection/lifecycle change is made.

## Restoration checklist

1. Obtain explicit owner direction identifying one or both sections.
2. Improve the owning QML/catalog surface; do not reimplement the backend.
3. Set the corresponding gate to `true`; update the default-hidden assertions
   in `tests/test-native-main-panel.js` / `tests/test-native-ping.js` deliberately.
4. Run main-panel, ping, traffic, connection-test, localization and QML gates.
5. Install the exact candidate and check English/Russian connected and
   unavailable states, layout, Tab order, close/reopen and bounded sampling.
6. Record actual evidence and update this document. Unavailable measurements
   must not be presented as proof that the user's VPN connection has failed.
