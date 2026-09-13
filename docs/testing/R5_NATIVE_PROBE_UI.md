# Native subscription probe frontend

This checkpoint wires the existing selected-subscription screen to the native
owner's bounded probe job. It must be installed only together with the executor
and registered runtime methods; no independent runtime activation is implied.

Fixed CLI and launcher mappings:

- `subscription probe INSTANCE OPERATION SUBSCRIPTION REVISION`
  → `subscriptions.probe` with exact instance, operation, internal record ID and
  expected revision. No URI, endpoint, timeout or arbitrary method is accepted.
- `subscription probe-results INSTANCE OPERATION`
  → `subscriptions.probe_results`, an explicitly private UI read.
- Existing `operation get/cancel` commands carry progress and cancellation.

The job shares the existing one-batch UI and does not use `nativePending`;
Disconnect remains independent. Closing the panel does not stop a daemon job.
Lost start replies retry the same operation, never a newly generated ID.

Unlike subscription/provider updates, successful probe jobs do not increment
the canonical revision. On terminal success the UI reads results through the
fixed command, then validates instance/revision, exact current subscription
membership, complete unique row coverage and every bounded result field.
The private result parser is capped at 64 KiB / 256 rows, independently of the
8-KiB ordinary operation projection. Missing/malformed/stale results show a
bounded localized error, not fabricated unreachable-server rows. Retrying a
failed result read only repeats that same private lookup.

Measurements populate the existing session-only `profileProbes` and completion
time cache. Owner/revision changes invalidate them. Default profile order is
unchanged; explicit latency sorting keeps active/favorite priority and failed
checks last. IDs remain internal private UI data, never shareable evidence.

English/Russian labels reuse the existing catalog except the new bounded
`native.probe.dns_failed`. Milliseconds, unavailable/error labels and all batch
controls use plain-text sinks. The Settings migration notice no longer claims
that subscription latency testing is absent.

## Acceptance boundary

Focused Node tests exercise fixed argv, same-revision success, result collection,
duplicate/missing/impossible/stale rows, maximum row count, retry, sort order,
unchanged refresh/provider behavior and no general mutation-slot occupation.
Launcher regressions cover exact argument count and no private error echo.
A Rust semantic-CLI unit test is included for the combined runtime build; this
frontend-only worktree deliberately does not compile another large runtime copy.

Required combined exact-head Try Omarchy visual states: EN/RU selected
subscription, Test running, cancellation, successful measured rows, unavailable
and DNS-failed row labels, latency sorting, constrained-height scrolling,
keyboard focus, close/reopen, and adjacent Settings/Subscriptions screens.
Inspect private captures locally; do not commit or publish real fixture images.
These visual/live gates are not established by the source contracts alone.

Local checks for this frontend checkpoint: native main-panel tests 20 passed;
native batch tests 14 passed; unchanged native subscription tests 13 passed;
launcher tests 23 passed; localization and QML contracts passed; shell syntax,
Rust formatting and whitespace checks passed. The new Rust semantic-CLI test
must still run in the combined runtime build.
