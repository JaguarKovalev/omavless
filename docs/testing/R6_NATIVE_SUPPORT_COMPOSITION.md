# Native support snapshot composition

Local R6 source checkpoint, 2026-09-11. This extends the fixed
`omavless diagnostics export` / `diagnostics.export` read; it is not a new
doctor command, host mutation or declaration of complete R6 acceptance.

## 2026-09-12 host/configuration completion (schema 3)

The current source extends the same ownership-fenced report with a strict
schema-3 `host` section. The frontend continues to accept schemas 1 and 2;
older clients fail closed on schema 3. Install the matching frontend/runtime.
The historical schema-2 evidence below remains valid for its exact source,
not installed acceptance of this new source.

The added fixed-size projection contains:

- core executable presence, required file-network capabilities and the real
  `/dev/net/tun` device shape;
- loaded/active/enabled facts for exactly `omavless-runtime.service` and
  `omavless-login-prepare.service`, plus whether the runtime service's MainPID
  is the current daemon (the PID itself is never returned);
- regular, non-symlink store/template/generated-config and package unit-file
  presence; private files must also be current-user owned and private-mode;
- configured-file rule/provider counts with the explicit `template` or
  `active_config` basis. These are not loaded-controller/provider-download
  counts and do not change `coverage.loadedPolicyCounts` to true.

Only fixed booleans, nullable booleans, bounded counts and enum literals leave
the collector. Unavailable service queries or unreadable facts are `null`, not
false/zero. A successfully observed missing dependency is `false` and therefore
different from an unavailable observation. The existing `coreSetupVerified`,
`serviceEnablementVerified` and `fileReadiness` coverage fields mean their
respective facts were obtained, **not that all values are healthy**. In
particular passive capabilities do not prove a successful TUN creation or
authorization, enabled login preparation does not prove an actual autoconnect,
and daemon MainPID attribution alone does not establish service/core/TUN
ownership of a healthy VPN connection.

The collector executes no Mihomo command and makes no HTTP/controller/network
probe. It reuses the existing bounded fixed-child/pipe collector for two exact
`systemctl --user show` reads and `/usr/bin/getcap` against the already-resolved
core path, at most 250 ms each. Together with the existing local observation's
250 ms controller budget, this stays inside the five-second unary protocol
budget without extending it. No new dependency, IPC method, privilege, service
mutation, configuration write or lifecycle transition is added.

Reference comparison is actual `backend.diagnostics_payload`,
`core_setup_status` and `routing_status`, not an invented full DNS diagnostic.
The legacy report obtains policy counts from files, even while connected;
the new report names that provenance explicitly. Six credential-free cases
(all three bundled templates plus empty/basic/nested shapes) compare the Rust
counts against the actual Python `yaml_top_level_block`, `yaml_sequence_count`
and `yaml_mapping_count` helpers. Existing configuration-subset parity remains
unchanged. This test-only oracle does not enter the installed application path.

The old best-effort aggregate `conflictCount` is not recreated by subtracting
unattributed interfaces. Existing typed visible-core/TUN/owned-auxiliary facts
remain available and more explicit about what was actually observed. Full
service/core/TUN ownership, DNS/routes/internet verification and login execution
remain separate acceptance evidence, not claims made by this support report.
Neither old Python `diagnostics_payload` nor this support export establishes
working DNS, routing or internet reachability.

Source checks: 14 Rust tests selected by `cargo test -p omavless-runtime --lib
support` pass, including the six-case actual-reference comparison, unknown and
transitioning service facts, duplicate/malformed systemd properties, private
file/symlink refusal, schema-3 null handling and existing ownership/store/revision
fences. Sixteen JS report tests pass across schemas 1/2/3, exact nested fields,
bounds, unavailable versus negative facts, stale clients and clipboard handling.
Successful projections remain below 4 KiB. Installed schema-3 acceptance must
still use the rebuilt binary and matching frontend; no live result is inferred
from these deterministic checks.

## Schema and consistency

The original checkpoint emitted schema version 2, scope `native_support`. It retains the
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

That schema-2 report does **not** verify service ownership/enablement, TUN ownership,
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

**Initial automated GUI copy was unconfirmed.** Settings and the Copy report focus target
were reached through actual keyboard navigation. Synthetic activation did not
yield a readable report in the clipboard; the panel was subsequently closed.
One shell restart did not resolve this result. The evidence does not yet
distinguish an input-automation/focus problem from the real copy path, and does
not justify calling the button PASS or blaming stale QML caching. A direct
human click/status observation was requested. Private screenshots remain outside
Git and are not shareable captures. No production timeout, privilege or focus
behavior was changed to force this smoke to pass.

**Owner follow-up, 2026-09-11: Copy report PASS.** The owner confirmed that the
installed Copy report action works. This resolves the above GUI acceptance gap
for the installed `46b4413c6626eec677fbcc7d9cd6627934d8d99e` frontend/runtime.
The private clipboard contents were not posted or committed.

Final observed native state after the restart was Routing/disconnected,
manualRecoveryRequired false, core/auxiliary/TUN counts 0/0/0. No VPN transition
or DNS change was requested in this integration pass.
