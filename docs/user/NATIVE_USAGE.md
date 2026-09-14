# Native OmaVLESS: everyday use

For the accepted Rust-owned application with its matching Omarchy frontend.
Start with [native installation](NATIVE_INSTALL.md) if the runtime is not yet
installed and activated. This guide does not announce a marketplace release.

## Connect the intended profile

Use **Connect** directly beside the profile name. **Disconnect** appears on the
connected row; the connected profile is also named above the main controls, even
when its subscription is collapsed or filtered out.

Clicking a profile name selects it for the fixed management action bar below
the list. That selection alone does not switch the VPN. Favorite, rename,
editor, QR, export, details and delete act on the named management target.
Subscription-managed profiles retain their editing restrictions.

Use the main **Full VPN / Routing / Direct** buttons to choose the mode. A mode
change while connected can require OS authorization and a connection transition;
finish or cancel the current prompt before starting another operation. An
uncertain or rejected transition is not proof that the requested mode is active.

## Import and update subscriptions

The main clipboard and file buttons accept either one supported profile link
or one supported subscription URL, then open the corresponding confirmation.
They do not accept an arbitrary remote configuration file.

In the main profile list:

- The arrow beside a subscription expands or collapses its profiles.
- **↻**, beside the server count, means **Update server list**. It fetches that
  subscription's current feed; it is not a latency test or a VPN reconnect button.
- The normal completion/error message reports the result. Wait for completion
  before another mutation; an unknown outcome needs reconciliation, not repeated
  clicks.

Open **Subscriptions**, then open the subscription you want for its refresh,
edit, remove and test actions. **Update all** refreshes the subscriptions as a
batch. Removing a subscription also removes its managed profiles; use the
confirmation carefully. Provider access can fail on a particular network;
a failed fetch is not evidence that a new server list was saved.

## Language, navigation and diagnostics

Settings → **Language** cycles System, English and Russian. The change applies
in place without restarting the plugin, runtime or tunnel. Private profile and
provider names are not translated.

Tab / Shift+Tab move through the open panel's controls. Enter or Space activates
the focused control; arrows move the profile-list cursor. Escape backs out of
the current view or closes the panel. The search field filters profiles without
changing the connection. Legacy-only letter shortcuts are not a promise of
native keyboard parity; visible controls remain the primary action path.

Use Settings for routing tools, Mihomo diagnostics and **Copy report / Save
report**. Reports use a bounded privacy-safe projection. Raw CLI status and
configuration files can contain private metadata and are not shareable reports.
Main-screen Test and latency sections are intentionally hidden for later
improvement; their absence is not evidence of a broken core.

## Close, disconnect, Quit

| Action | Effect |
| --- | --- |
| Close panel / restart shell | Leaves the requested tunnel running |
| Disconnect | Stops the current VPN connection; keeps the plugin/runtime available |
| Settings → Shut down OmaVLESS / Quit | Confirms shutdown, verifies core/TUN cleanup, then stops/disables runtime and plugin; preserves private data and installed files |

If shutdown fails or needs recovery, do not interpret the closed UI as proof
of successful cleanup. See [native recovery](NATIVE_INSTALL.md#updates-close-quit-and-removal).

Login autoconnect is **Off by default**. Native Last/pinned fresh-login behavior
remains under validation; leave it Off unless deliberately testing it. Changing
the saved login preference does not disconnect the current session.

## Service identity

The native user service is `omavless-runtime.service`, not the legacy
`omavless.service`. The runtime owns Mihomo; the panel is a client. For a local
service check, use:

```sh
systemctl --user status omavless-runtime.service --no-pager
```

Do not share unreviewed service/log output. Package removal and frontend removal
are separate operations; the legacy `uninstall.sh --purge` is not a native
removal command. Follow the [native guide](NATIVE_INSTALL.md).
