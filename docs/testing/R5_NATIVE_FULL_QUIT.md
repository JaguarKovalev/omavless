# Local R5 full application Quit candidate

Owner-directed local work, 2026-09-10. Not merged or published. The current
marketplace version and V0/#30 implementation/evidence are unchanged.

## Ownership and semantics

Settings now has an explicitly confirmed **Shut down OmaVLESS / Quit** action.
It is distinct from closing the panel, closing a terminal, shell reload or a
lost client. Those actions never call the new shutdown path.

The Rust `runtime.quit` request accepts only `instanceId`, `expectedRevision`
and `operationId`. It reuses canonical native disconnect validation/replay and
ownership checks. An admission read/write barrier covers every unary handler,
including remote subscription completion: concurrent work returns busy rather
than racing disconnect with reconnect. The existing auxiliary lease is drained,
fresh disconnected/zero primary/auxiliary/TUN facts are required, background
work is revoked, and new admission is sealed. The runtime exits successfully
after bounded workers drain, so `Restart=on-failure` does not relaunch it.

The fixed `omavless plugin quit INSTANCE REVISION OPERATION` CLI authenticates
the installed daemon on the same socket used for the request: same UID, service
MainPID and installed executable inode. It verifies packaged unit identity,
no override/drop-ins, and agreement with the user manager's private path roots.
After graceful exit it acquires the runtime owner lock, verifies disconnected
desired state plus the strict empty-host predicate, disables the fixed native
user unit, repeats verification, and **only then** disables `kdk.omavless` via
the supported Omarchy command. No arbitrary command/service/path/PID input,
sudo, pkexec, process killing, package deletion, Python fallback, or startup
preference rewrite is added.

Quickshell destroys its direct `Process` child on unload (see the upstream
[Process destructor](https://github.com/quickshell-mirror/quickshell/blob/master/src/io/process.cpp)).
Only this confirmed action uses a waiting shell wrapper around the bounded
Rust child, allowing final verification to complete after plugin disable.
Other launcher commands retain their existing process behavior. There is no
short QML watchdog that kills a human authorization prompt; the native request
has a 120-second outcome-unknown bound. Unknown outcome never means successful
shutdown and never causes an automatic kill/retry.

Any failure before final plugin disable leaves the plugin present. A late
failure may leave a safely stopped runtime with an enabled frontend; this is
reported, not hidden or compensated by reconnecting. If the shell accepts
disable but its acknowledgement cannot be read, the result remains unconfirmed,
though VPN/runtime were already verified stopped.

Profiles, subscriptions, routing preferences and private startup settings are
preserved. Runtime auto-start is disabled at the systemd user-unit level. To
explicitly launch again on the installed Arch/Omarchy host:

```sh
systemctl --user enable --now omavless-runtime.service
omarchy plugin enable kdk.omavless
```

This does not replay the old connected request. Existing per-login receipt and
startup policy still govern a later fresh login; do not delete that receipt to
simulate acceptance. A user-friendly single-action reopening flow is separate
from this shutdown checkpoint.

## Static and deterministic evidence

- Rust runtime suite: 665 passed, six ignored; includes 11 new request,
  admission, connected exit, stale/invalid refusal, cleanup-proof and fixed
  host-ordering tests. The failure matrix covers all nine host boundaries.
- Full Python suite: 351 executed, four skipped (347 successful tests).
  Includes fixed Quit launcher arity, no Python fallback and harmless synthetic
  wrapper-destruction survival coverage.
- Five new native Quit JS tests exercise production QML function extraction,
  guarded action, confirmation/cancel wiring and EN/RU catalog lookup.
- Full invoked JS/QML contracts, clippy all targets with warnings denied,
  formatting, Python compile, shell syntax, manifest, plugin validate and
  whitespace checks pass.

These are local static results, not installed or human visual acceptance.
Six synthetic exact-source Quickshell renders (EN/RU Settings, confirmation and
failure) were inspected locally. The failure text exposed a RowLayout height
issue; SettingsActionRow now publishes its content-derived implicit height,
and all six affected/adjacent states were recaptured without overflow. Captures
use synthetic metadata and a no-op backend outside Git; they cannot prove live
shutdown or human keyboard interaction.

## Remaining acceptance at implementation checkpoint

- Exact packaged candidate identity and installed runtime restart.
- Installed disconnected and connected Full Quit: runtime/core/TUN gone,
  plugin disabled only after proof; no automatic restart; explicit reopening.
- Exact EN/RU settings, confirmation/cancel and error rendering, keyboard
  navigation and human observation.
- Explicit authorization rejection where reproducible; UI must stay visible.
- Ordinary close/reload must retain a requested healthy tunnel.

Direct Omarchy disable/remove lifecycle watchers remain a separate gap: this
explicit button does not claim that those entry points are now ported. Final
fresh-login, upgrade/rollback, all-surface Python-unavailable R6 acceptance and
the [documented DNS/provider follow-up](R5_NATIVE_SUBSCRIPTION_PROBES.md) remain
open. This checkpoint does not declare R5/R6 or V0 complete.
