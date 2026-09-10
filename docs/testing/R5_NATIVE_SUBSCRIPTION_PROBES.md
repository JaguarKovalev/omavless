# Native subscription latency — local integration candidate

This local R5 candidate combines the pure probe plan, private DNS resolver,
auxiliary-core lease, bounded controller executor, shared operation owner and
the existing selected-subscription UI. It is not R6 retirement or marketplace
acceptance. No main merge or remote publication is authorized for this work.

## Executable boundary

`subscriptions.probe` admits only an existing internal subscription ID with
instance/operation identity and expected revision. It shares the one existing
batch slot and bounded remote-work pool; there is no second mutation registry.
The fixed semantic CLI and launcher expose no URL, path, command or timeout.

The owner captures the private store, desired state, template and optional
active configuration under its normal lock. DNS uses active-config policy first,
then the template, without inventing resolvers or invoking libc DNS. A batch
caches bounded public pins per endpoint, then renders the reference-compatible
plan: at most 256 profiles, four addresses each and 64 aliases per core chunk.
Unresolved DNS is distinguished from a measured unreachable server.

DNS, pool waiting, controller calls and process cleanup run outside the runtime
dispatcher lock. The whole job is bounded at 30 minutes, with bounded per-host,
controller and cleanup deadlines. Progress stays running until final validated
rows; it does not misrepresent DNS completion as completed latency measurement.
The frontend automatically polls for up to ten minutes, then requires an
explicit same-operation check rather than silently restarting work.

One revocable auxiliary lease owns the disposable Mihomo child. It creates no
TUN, inbound listener or TCP controller. The requested VPN remains independently
owned. Raw observation includes the auxiliary process count, while lifecycle
health subtracts only the exact registered/proven owned child. Unrelated cores
are never exempted by name. A genuine mutation revokes/drains the lease before
host effects; invalid/stale requests and cached replays do not cancel new work.

The normal three reference HTTPS targets and latency-merging semantics remain
unchanged. A failed runner/controller exchange is a job failure, not fabricated
unreachable rows. Cancellation and every normal error path reap the child before
terminal publication. Failed cleanup marks the owner manual-recovery-required.

Committed native startup reconciles only marked orphan probe directories before
starting a successor core, while holding the existing ownership/migration locks.
Cleanup requires strict core/TUN absence, dead creator PID, private directory and
marker, a fixed bounded file allowlist and a nonresponsive stale Unix socket.
It pins directory/file identities and refuses unknown data. This is not a
general cleanup command or recursive deletion by prefix.

## Results and UI

The owner publishes only a complete, revalidated snapshot. Results remain in
memory (last 16 successful jobs), never update the private store or canonical
revision, and are read through the explicit private `subscriptions.probe_results`
method. The response includes internal IDs for local UI mapping, not support
output. Stale instance/store/template/active-config/desired/revision is refused.

Selected-subscription Test, common progress/cancel controls, measured latency,
DNS failure and unavailable labels reuse the existing UI. Optional latency
sorting retains active/favorite priority; the default order is unchanged.
Disconnect does not depend on the test's UI busy flag. EN/RU use shared stable
keys and plain-text rendering. Closing a panel does not create another job.

## Acceptance status

The component checkpoints remain documented in `R5_NATIVE_PROBE_PLAN.md`,
`R5_AUXILIARY_CORE_OWNER.md`, `R5_NATIVE_PROBE_CONTROLLER.md`,
`R5_NATIVE_PROBE_EXECUTOR.md`, `R5_NATIVE_PROBE_RESOLVER.md`,
`R5_NATIVE_PROBE_OWNER.md` and `R5_NATIVE_PROBE_UI.md`.

Final local runtime command: 654 passed across 14 result groups, six opt-in
tests ignored, no failures (600 library tests). Strict all-target clippy passes.
Python reference command: 345 tests, four skipped; all invoked JS suites and
QML/localization contracts pass. Focused native batch/main-panel tests: 15/20.
Actual Mihomo 1.19.30 linux/arm64 opt-in renderer/private-controller tests: two
passed. Actual Quickshell import-graph compilation passes for Panel and Service;
qmllint is unavailable. Shell syntax, manifest and plugin validation pass.

Still pending for this exact combined candidate: package identity,
installed real subscription measurement/cancel/disconnect,
Unix-only controller/process/TUN invariants, scoped crash cleanup, privacy audit
and actual EN/RU visual states. Do not infer these from deterministic tests.
