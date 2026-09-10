# Native subscription probe owner boundary

This local checkpoint is not live registration. It supplies the owner half of
the existing isolated-probe plan. It never launches Mihomo, resolves DNS, opens
a controller, writes configuration or changes the requested VPN connection.

## Reference and intentional changes

Python `backend.py:probe_subscription` and `probe_subscription_stream` select
the existing subscription's non-missing managed profiles, then return positional
`resolved`, `reachable`, and `latencyMs` values mapped to internal record IDs.
They do not save probe results. `Service.qml:profileProbes` and
`subscriptionProbeTimes` are explicitly session-only UI caches.

Rust therefore **does not write the private store or increment canonical
revision on successful probes**. The new `SubscriptionProbe` operation is the
only nonempty-success exception to the registry's ordinary base-revision-plus-
one rule. Subscription refresh and provider refresh retain their old rules.
Existing pure-plan differential tests remain the renderer/aggregation oracle;
this owner checkpoint tests admission, snapshot selection and lifetime fences.
It is not a new provider-interoperability claim or permission to remove Python.

## Private snapshot and bounded API

- `subscriptions.probe`: exact `instanceId`, `operationId`, `subscriptionId`,
  optional `expectedRevision`; no URL, profile list, timeout, shell or path.
- `operations.get` and `operations.cancel` retain their fixed safe shapes.
- `subscriptions.probe_results`: exact instance/operation lookup, explicitly
  private UI response `{version:1, subscriptionId, results:[{id, resolved,
  reachable, latencyMs}]}`. Names, endpoints, credentials and controller paths
  are absent. Internal IDs must not be copied into shareable diagnostics.

The existing validated domain store is consumed into only the selected current
canonical profiles and IDs. Missing records and unrelated members do not enter
the worker. At most 256 results use the established 0..60000 millisecond or -1
unreachable sentinel. The fixed private route template is carried to the worker
for the future resolver policy, never returned over this API.

One probe shares the same active slot, operation collision namespace, exact
retry behavior, cancellation registry and supervisor ticket as refresh-all and
provider work. Snapshot fencing checks exact private-store/template hashes,
desired state/generation, canonical revision and current ownership. Worker
progress and final completion must re-enter the serialized owner; network and
child cleanup run outside it.

Only the last 16 successful batches are retained in daemon memory. Older or
unretained results return not-found, stale snapshots conflict, and daemon
restart discards all results. No hidden next-login server selection is made.
Terminal operation metadata retains the existing independent registry limit.

## Cleanup and admission integration requirements

The scheduler must retain `supervisor_ticket()` before spawning and abort lost
work. It must reap the auxiliary child before completion. Failed or uncertain
cleanup is `manual_recovery_required`, blocks the shared owner and exposes that
state in its ordinary actual-state projection. It must not become a mere
unreachable-server result, even when cancellation was also requested.

`mutation_operation_known` is only a pre-effect optimization: while retaining
the owner lock a known ID can replay/conflict without cancelling an unrelated
probe. It is not authorization. Do not release the owner lock between this
check and ordinary dispatch; cache eviction can invalidate the observation.
Unknown IDs require proper auxiliary revocation/drain followed by full admission.

The executor, trusted auxiliary identity, process-reap barrier, controller and
resolver policy, fixed CLI, QML integration and exact installed active-VPN plus
disconnect/restart acceptance are separate requirements. This owner boundary
must not be activated alone.

## Local deterministic evidence

- Runtime library: 558 passed, five pre-existing opt-in tests ignored.
- Five focused owner tests pass: volatile success/replay, cancellation/failure,
  invalid rows, exact store/template/desired/revision/shutdown fencing, shared
  namespace, stale reads, uncertain cleanup, bounded retention and ticket abort.
- One selected-probe protocol test passes strict input, private error non-echo,
  subscription-specific digest and exact result-lookup coverage.
- One domain snapshot test passes current/missing/unknown subscription selection.
- Strict runtime all-target clippy, workspace formatting and diff check pass.
- After final stopped-read guard and additional desired-state/known-ID assertions,
  all five owner tests and strict clippy pass again.

These are synthetic, no-provider tests. No installed plugin or VPN state changed
for this boundary. Live acceptance remains the combined executor's gate.
