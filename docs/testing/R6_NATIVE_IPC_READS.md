# Native cached shell IPC reads

Local-only source checkpoint, 2026-09-11. No package install, plugin reload or
VPN/authorization operation was performed for this change.

`Panel.qml`'s existing `status`, `routing`, `details` and `diagnostics` shell IPC
methods now share the pure `NativePresentation.ipc` projection. They no longer
return the early migration's unconditional read-only/unavailable placeholders.
Legacy branches are unchanged. No Rust method, timer, background query or
mutation is added; these commands read already parsed UI caches only.

All prose explicitly says cached/not verified. Desired mode is not an effective
route assertion. Instance/revision/generation/mode/actual-state mismatches,
unavailable facts, failed metadata, pending actions and unknown outcomes cannot
be presented as a healthy cached connection. Manual recovery takes precedence.
Unavailable counters/booleans are null, never fabricated zero/false host facts.

`diagnostics` returns `schemaVersion:1`, `scope:cached_native_ipc`, bounded
counts, fixed enums/booleans/nulls and explicit false verification coverage for
live health/routes/DNS/internet. No profile/provider names, IDs, endpoints,
credentials, controller paths, errors or private detail payloads are included.
`details` is a small local resource summary, not a replacement for the explicit
private profile-details command or the full legacy support report.

The strings are fixed English CLI diagnostic fallbacks, not UI catalog keys.
No screen labels, layout or localization behavior changed. Thirteen pure
projection/privacy cases plus one test executing the actual QML handler bodies
pass; seven existing native presentation cases and the existing IPC mutation/
resolution tests also pass. The old source contract now checks the shared
projection binding; behavioral tests enforce its safety semantics.

Installed shell IPC acceptance is pending until an explicitly safe frontend
deployment. Source tests are not a claim that the user's running plugin changed.
Headless `importConfig(path)`, full support composition, new-user initialization,
login/package recovery and installed Python-unavailable R6 gates remain open.
