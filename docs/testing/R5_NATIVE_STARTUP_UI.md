# R5 local native startup-settings frontend

This local candidate restores the existing Settings / Start at login dialog for
the native owner. It is not installed acceptance and does not complete R5/R6.
Python remains the legacy owner/oracle; this branch does not remove Python.

## Boundary

The existing `StartupPrompt.qml` supplies English/Russian labels and the same
last-profile / selected-profile and Routing / Full VPN choices. Native profile
metadata remains private and untranslated. Saving uses a fixed
`plugin startup-configure` CLI action with four bounded stdin lines; no profile
record is placed in argv. The Rust bridge reuses the canonical startup parser.
The mutation retains the current daemon instance, expected revision and operation
ID, including an exact retry after an uncertain result. There is no optimistic
store edit, connection change, unit enable, shell command or Python fallback.

The Edit action appears only when the current native capability projection
advertises `startup.configure`. Capability reads are bounded, timeout after six
seconds, and refresh only while Settings or its startup dialog is open. A
revision/instance change invalidates that presentation hint. The server's
instance/revision/host admission remains authoritative; a capability response
is not itself proof that a mutation can succeed. Stored preferences alone never
claim that the user service is enabled.

The companion local login-integration candidate must provide the actual
registered action and host admission before this UI can be accepted. Against an
older daemon, Settings remains read-only instead of exposing a dead button.

## Local checks, 2026-09-10

- 14 focused Rust plugin-action tests passed; strict all-target runtime clippy
  and workspace formatting passed.
- 7 new native startup UI checks passed, including stdin bounds, canonical
  parser reuse, stale/failed capability reads, exact retry, missing selections,
  action-result fencing and shared-dialog wiring.
- Reference suite: 345 Python tests, 5 skipped in the restricted run; all
  invoked JS suites, localization contracts and QML contracts passed.
- Shell syntax, manifest JSON, diff check and Omarchy plugin validation passed.
- Actual installed Quickshell/Wayland imports compiled `Panel.qml`, `Service.qml`
  and `StartupPrompt.qml` without creating those components or starting a VPN.
  An offscreen-only attempt could not load the required PanelWindow backend;
  it was not counted as product failure or visual acceptance.

The localization skill requires exact installed English/Russian visual checks
after host integration. Those checks, saving while connected without reconnect,
ordinary daemon restart, fresh-login behavior and restoration remain pending.
No unit, installed package, private store, startup preference or GitHub branch
was changed by this frontend checkpoint.
