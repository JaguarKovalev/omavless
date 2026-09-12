# Selected versus connected profile — bounded UI correction

Local candidate `d56b2e2` on `codex/local-native-probe-execution`.
Try Omarchy ARM64 VM; no push, merge, runtime replacement or R6 completion.
Installed runtime remains source `7b75b74`; only QML/presentation/catalog change.

## Confirmed findings and changes

1. **Selection looked like connection.** The old filled profile dot represented
   only the action cursor, while the header omitted the active profile and a
   collapsed subscription could hide it. Selection now has an arrow and explicit
   "Selected for actions" copy. A selected row exposes a labelled Connect or
   Disconnect action; clicking its name still only selects it. Mouse and existing
   Enter activation share the same fenced semantic-action helper. This does not
   silently convert navigation into a VPN transition.
2. **Active identity was hidden.** A separate plain-text connected-profile line
   derives only from the verified presentation's active ID. Connected rows and
   collapsed owning subscriptions have an explicit Connected label, not color
   alone. Search/selection never replaces active identity. Unavailable states
   do not retain an active claim, and selection copy does not falsely promise
   disconnection when actual state is unknown.
3. **Header QR could target a different selected profile.** While connected it
   now targets the active record and says so in its tooltip; row QR remains
   scoped to its row. While disconnected the header uses the selection; unknown
   state refuses rather than guessing a target.
4. **Power admitted unavailable selections.** Its disabled state and action
   guard now reject missing/removed selected records without falling back to a
   different last-used profile. Normal connected power behavior remains explicit
   Disconnect. Mode buttons still use the existing fenced native mode action;
   no host-authorization/rollback repair is claimed here.

## Evidence and limitations

| Step | Check | Result |
| --- | --- | --- |
| 1 | Installed disconnected main, English | New cursor displayed; disconnected status and modes remain legible |
| 2 | Arrow-select a profile without Enter/Connect | "Selected for actions" plus explicit Connect appears; no VPN action, no clipping/overlap in captured viewport |
| 3 | Switch locale to Russian in place | Equivalent selection/connection copy and button fit; provider/profile text stays untranslated; normal System locale restored |
| 4 | Active profile differs from selected; active group collapsed/filtered | Production-function and QML contract tests PASS; connected-state installed visual capture NOT RUN in this pass |
| 5 | Duplicate names, unavailable/transition states, stale/missing selections and header/row QR targeting | Deterministic regressions PASS; no actual connection or QR credential export initiated |

Private screenshots were inspected locally and are not publishable. The initial
post-install capture still showed the old cached dot; a shell-only restart was
required. Subsequent stable captures show the new installed implementation.
The native runtime PID (576) survived and remained Routing/disconnected with
Mihomo/TUN/auxiliary 0/0/0, recovery false, and plugin enabled. All 20 tracked
plugin/launcher runtime files match the checkout byte-for-byte.

Reference Python suite: **460 tests, four skipped, PASS**. The first integrated
runner then stopped at an outdated isolated JS test fixture that did not load
the newly shared action helper. The fixture was corrected; all JS suites were
rerun, including **34 main-panel**, **8 presentation**, IPC, catalog and the real
QML file-export Process test. QML contracts, plugin validation, Python compile,
shell syntax, manifest JSON and diff check PASS. No Rust production source
changed, so earlier Rust gates were not repeated ceremonially.

This is a bounded correction and adjacent-state review, **not proof that the
entire UI has no remaining defects**. Real connected-state English/Russian
layout, actual button-driven profile switching and authorization cancellation
remain distinct checks. Keep the candidate local and R6 open until its existing
acceptance gates are fulfilled. The owner later noted that the latest agent
transport interruption likely occurred outside the VM; do not classify that
message alone as another confirmed VPN failure.
