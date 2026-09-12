# Installed native QML bridge without Python

Try Omarchy ARM64, 2026-09-12. Local-only acceptance; no push, main merge or
R6-complete claim is made by this report.

## Exact installed identities

- Runtime/frontend source: `7b75b747883d66a05f1f2b321d42f194c6040b5c`.
- Package: `omavless 0.0.0.r492.g7b75b747883d-1`, aarch64.
- Executable SHA256:
  `fe04fba32d4e135d56e929350cd0296d3f89168e06d6ebba9a6de089aec4b786`.
- Archive SHA256:
  `1a6edabad975d8af7cde768019af15e4dc8fc103db440090f5e27219b7a18b32`.
- Test-tool source: `4712d69`; 19 installed `plugin/` files match the current
  source bytes. The installed frontend has no `backend.py`.

Normal pacman dependency checks installed the package. The already running
owner's daemon was not silently treated as upgraded: its retained process was
still the prior binary. Actual new-binary execution below is independently
proved in the disposable account and its real packaged systemd user service.

## Isolation and actual production path

The reviewed disposable-account wrapper creates only an absent, locked
`omavless-r6-domain` account. A temporary root-owned `user@UID.service` drop-in
makes the real interpreter target inaccessible in that user manager's mount
namespace. Both PATH lookup and absolute interpreter aliases are negatively
tested. The actual CLI, native daemon and offscreen Quickshell all execute in
that namespace. This is not a synthetic daemon/socket or a Python-process survey.

The actual installed plugin payload is copied without modification to that
account's normal plugin location. `installed_native_bridge.qml` instantiates
its real `Service.qml`; it neither replaces Service functions nor supplies
fake backend responses. File-content input and confirmations use real Service
actions, installed `backend.sh`, `/usr/bin/omavless`, the authenticated private
socket, real native owner and atomic private writer.

A bounded Node HTTP server serves only a credential-free reserved-address
synthetic subscription on loopback. Exactly three requests were observed:
add, refresh and update. This exercises the actual production HTTP/decoder/
transaction path, not private-provider interoperability. Node is test tooling,
not an OmaVLESS dependency; its existing mise executable was copied to the
strictly guarded private test-tools directory.

The first attempt stopped before QML because the harness assumed `/usr/bin/node`.
It is not evidence of a product failure. The corrected fixed-location fallback
was checked before rerunning; no runtime timeout or security rule was relaxed.

## Observed matrix

All 12 installed domain stages passed again, followed by these 16 QML stages:

| Stage | Result |
| --- | --- |
| Native Service readiness and empty baseline | PASS |
| Profile file preview and explicit confirmation | PASS |
| Imported profile persisted | PASS |
| Subscription URL file preview and confirmation | PASS |
| Subscription add via actual HTTP | PASS |
| Subscription refresh via actual HTTP | PASS |
| Private subscription editor read | PASS |
| Subscription update via actual HTTP | PASS |
| Subscription deletion | PASS |
| Custom routing rule add | PASS |
| Custom routing rule read | PASS |
| Custom routing rule deletion | PASS |
| Profile export through native writer | PASS |
| Schema-3 support report through matching parser and writer | PASS |
| Standalone profile deletion | PASS |
| Final disconnected native state | PASS |

Both exports were regular non-symlink files owned by the test user, mode0600.
Profile bytes matched the synthetic input; report schema/scope validated.
No profile/URL/result contents enter this report or Git.

After Quickshell shutdown, the real removal watcher had time to resolve the
absent shell registry as Unknown, not Disabled. The native daemon remained
available. A subsequent real service restart preserved the synthetic store;
the actual process executable matched the package digest and inherited the
Python mask. Observed state was disconnected, recovery false, core/TUN/
auxiliary counts0/0/0. Account/linger/home and the exact temporary namespace
configuration were cleaned successfully.

## Evidence limits

This closes representative installed QML/domain/subscription/diagnostic bridge
coverage under actual Python absence. It does not simulate a visible file
picker, clipboard, real provider, VPN connection or login Last/pinned. Existing
exact installed visual/helper evidence is complementary; a Cartesian product
of every locale/dialog with this namespace is not required.

Fresh connected login and live lifecycle under Python absence retain their own
host gates. No OS authorization policy, controller exposure or network setting
was changed. The owner requested no further file-dialog automation in this
session; additional cancel/overwrite experiments are not presented as failures
of this passing native bridge.
