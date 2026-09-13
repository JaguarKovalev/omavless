# Installed native startup Off with Python inaccessible

Try Omarchy ARM64/virtualized, 2026-09-11. **PASS for the bounded empty-user
installed gate below; not complete R6, graphical login or live VPN acceptance.**

## Exact installed identity

- Source `82a4444880bdeefcc00e64d5d0556488212d9be6`.
- Arch package `omavless 0.0.0.r469.g82a4444880bd-1`, aarch64.
- Installed and actually executed daemon binary SHA-256:
  `c8a5b4883aeed2672c3f3bfa55d055c3cec7f7acf439b09c2da1a368c6e7d639`.
- Ordinary package installation/dependency evidence is retained in
  [the fresh-package report](R6_FRESH_PACKAGE_ACTIVATION_2026-09-11.md).

The main desktop's older running daemon is not attributed to this identity.
This gate executed the installed binary in a disposable account and verified
its running `/proc/PID/exe` digest, not merely the file on disk.

## Isolation and real host boundary

A one-shot reviewed script ran with sudo in a visible terminal. It created a
locked disposable user with an empty home and a minimal `default.target` that
requires `basic.target`; unrelated Omarchy desktop services were not started
for this account. It used the real systemd user manager, real bus, packaged
native units and actual `/usr/bin/omavless`, not a fake systemctl or socket.

A temporary root-owned drop-in for **only the disposable `user@UID.service`**
used `InaccessiblePaths=/usr/bin/python3.14`. The installed `python` and
`python3` symlinks both resolve there. CLI commands entered that same mount
namespace before dropping to the test UID. Negative tests proved both PATH
lookup and all three absolute interpreter entry points unavailable. The actual
daemon inherited the same namespace. Main-user and host Python stayed intact;
no interpreter package was removed, no capability/OS policy relaxed and neither
packaged OmaVLESS unit was overridden. This is deliberate interpreter absence,
not just a restricted PATH or a trace with no observed Python execution.

This does not prove absence of arbitrary copied interpreters on other hosts.
The main desktop, private store and network configuration were not used.

## Observed matrix

| Check | Result |
| --- | --- |
| Python PATH and absolute-alias negative controls | PASS |
| Fresh `setup initialize`, startup Off, empty store | PASS |
| `cutover activate` with no legacy unit | PASS |
| Real installed snapshot, observation and schema-1 support export | PASS |
| First ordinary committed native restart consumes login receipt | PASS |
| Complete disposable-user teardown and new user-manager session | PASS |
| New manager epoch, exact domain-separated epoch hash in consumed receipt | PASS |
| Native unit starts in new session, Routing/disconnected, core/TUN 0/0 | PASS |
| Same-manager runtime restart preserves receipt and disconnected state | PASS |
| Private store bytes preserved through login checks | PASS |
| Running daemon digest and Python-masked namespace | PASS |
| Disposable account/home/runtime directory and mask cleanup | PASS |

Cutover's preparing candidate intentionally skips login preparation. The test
therefore performs a normal committed runtime restart before measuring the
first consumed receipt. A new session is created by disabling linger,
terminating the disposable user, waiting for **automatic** `/run/user/UID`
teardown, and starting a new manager. No receipt is manually removed to force
success. Simply restarting `user@UID.service` can retain the runtime directory
and stale receipt; that is not this successful fresh-session gate and must not
be relabelled as a login test. Existing epoch-mismatch refusal stays intact.

The local script/stdout are retained outside Git under the session's
`epoch-no-python` build-artifact directory. Only the sanitized matrix is
shareable. There were no private profiles, real endpoints or network probes.

## Remaining R6 gates

This closes only fresh initialization/activation, startup **Off**, and
same-session disconnected restart for a real installed daemon with Python
inaccessible. It does not close Last/pinned login, graphical QML/helper flows
under interpreter absence, real subscription/routing/connect/disconnect,
controlled DNS authorization, package removal/upgrade/rollback/recovery or
full host support-report parity. Those gates remain explicit in the
[dependency audit](R6_PYTHON_DEPENDENCY_AUDIT.md).
