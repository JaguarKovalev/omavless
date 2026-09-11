# Native support snapshot composition

Local R6 source checkpoint, 2026-09-11. This extends the fixed
`omavless diagnostics export` / `diagnostics.export` read; it is not a new
doctor command, host mutation or declaration of complete R6 acceptance.

## Schema and consistency

The emitted result is schema version 2, scope `native_support`. It retains the
accepted counts/preferences configuration subset from
[the original support checkpoint](R5_SUPPORT_DIAGNOSTICS.md) and adds a bounded
`localObservation`. The QML clipboard/file report parser accepts both the old
schema-1 configuration report and the strict new schema; an old frontend
rejects the new schema instead of misinterpreting it. Install matching frontend
and binary for the new report. Unknown fields and future schema versions refuse.

The native owner holds the existing migration lease, verifies exact committed
ownership, validates the complete private store, captures desired state and
pending-routing status, and invokes the existing bounded `fresh_observation`
host boundary. Desired state, store bytes, routing-pending state and ownership
are checked again before release. A changed input refuses the whole report;
an observation error instead produces `availability: unavailable` and `facts:
null`, while the validated configuration remains useful. No unavailable sample
is rendered as healthy zero process/interface counts.

Only fixed categories, bounded integers and booleans are emitted. Fresh facts
are owned-core liveness, visible Mihomo/TUN counts, the owned auxiliary-core
subset, authenticated owned-controller configuration verification and matching
desired-profile verification. Desired mode/connected are included, but profile
IDs, desired generation, daemon instance, private paths, credentials, provider
names, destinations, logs and raw errors are not. Cached lifecycle remains
`lastKnownState`; a fresh local sample never promotes it to connected/healthy.

`coverage.liveHostObservation` means the typed local observation succeeded.
`coverage.controllerQuery` is true only for positive owned-controller config
verification; false includes not attempted or unavailable, not proof of a bad
controller. Inventory saturation/inconsistent combinations produce an unavailable
sample. The same exact shapes, bounds and consistency predicates are enforced
by the QML parser, which constructs a fresh shareable object rather than copying
the raw response. Normal revision/instance freshness gates in Service.qml remain.

## Honest differences from the Python reference

`backend.diagnostics_payload` remains the actual configuration-subset oracle.
The existing 26-case differential corpus is preserved unchanged. Native owned
controller proof is stronger than the old support report's socket-exists check;
visible TUN count is deliberately not called `tunReady`.

The report does **not** verify service ownership/enablement, TUN ownership,
DNS, routes, internet reachability, login activation, core installation/setup,
file readiness, loaded routing-rule/provider counts or aggregate conflict count.
Explicit false coverage/verification fields preserve these limitations. Stored
preset preference is not observed loaded policy. Separate existing setup,
live-rule diagnostics and private connection-details UI remain distinct reads;
no private details are merged into shareable support output.

This is a reviewed bounded support snapshot, not byte-for-byte complete legacy
doctor parity. A complete host support bundle or product-contract acceptance
of the narrower replacement remains a separate gate; this source checkpoint
does not silently erase it from R6.

## Deterministic checks

- Nine focused Rust unit/socket/CLI tests pass, including schema, request
  privacy, invalid/unsafe store, ownership-generation fencing, observation
  failure/null and desired/store/ownership changes during observation.
- Twelve JS support-report tests pass, covering schema-1 compatibility,
  observed/unavailable schema-2 results, nested unknown-field rejection,
  unearned verification, incompatible schemas, bounds, stale UI context and
  clipboard lifecycle.
- Successful report size remains below 4 KiB with inventory cardinality not
  increasing output size. Synthetic credentials never enter shareable output.

Integration checks: the full locked Rust workspace passed 949 tests with ten
explicit opt-in skips; strict runtime all-target Clippy and formatting passed.
The serial reference run passed 411 Python tests with four skips, all JS
surfaces and QML contracts. A prior concurrent run hit three short Python
test timeouts while Cargo was busy; the complete serial rerun passed without
changing timeout or production code. The final CLI-help-only adjustment also
passed all 22 CLI tests. Shell syntax, Python compile, manifest JSON, plugin
validation and diff checks passed.

Installed schema-2 report-copy acceptance remains distinct from these source
checks. No live VPN, authorization or private-fixture acceptance is claimed
by this implementation slice.
