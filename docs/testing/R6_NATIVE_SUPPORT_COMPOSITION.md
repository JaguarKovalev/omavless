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

## Installed integration, 2026-09-11

Runtime/package and frontend source:
`46b4413c6626eec677fbcc7d9cd6627934d8d99e`. The aarch64 package is
`omavless 0.0.0.r471.g46b4413c6626-1`; executable SHA-256 is
`b4abb18b88f91b12071b6608db07e77046093df2922966f842ef4fd4216ff2c6`.
It was installed with ordinary pacman dependency checks. All 19 frontend files
matched the checkout after `install.sh --native-only`; no legacy backend or
uninstaller was installed. A disconnected service restart loaded the new binary;
the actual `/proc/PID/exe` digest matched, not merely the disk artifact.

The actual native `diagnostics export` emitted schema 2, scope `native_support`,
with an observed disconnected Routing state and core/TUN counts 0/0. Running the
installed matching parser against that response accepted a 1515-byte shareable
projection. Forbidden URI/UUID/identity/path-field patterns were absent. These
checks are successful CLI/projection integration, not GUI clipboard acceptance.
The same executable passed 55 synthetic CLI cases with Python masked; the
separate real empty-user Off/login gate above used the prior `82a4444` binary.

**GUI copy remains unconfirmed.** Settings and the Copy report focus target
were reached through actual keyboard navigation. Synthetic activation did not
yield a readable report in the clipboard; the panel was subsequently closed.
One shell restart did not resolve this result. The evidence does not yet
distinguish an input-automation/focus problem from the real copy path, and does
not justify calling the button PASS or blaming stale QML caching. A direct
human click/status observation was requested. Private screenshots remain outside
Git and are not shareable captures. No production timeout, privilege or focus
behavior was changed to force this smoke to pass.

Final observed native state after the restart was Routing/disconnected,
manualRecoveryRequired false, core/auxiliary/TUN counts 0/0/0. No VPN transition
or DNS change was requested in this integration pass.
