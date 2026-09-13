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
Actual Mihomo 1.19.30 linux/arm64 opt-in tests: four passed (two renderer/private
controller tests, native host validation and supervisor validation).
Actual Quickshell import-graph compilation passes for Panel and Service;
qmllint is unavailable. Shell syntax, manifest and plugin validation pass.

## Installed Try Omarchy ARM64 evidence — 2026-09-10

Tested code: `9bf5781c6c2ef20168cb25a25e8a6ae8ec427065`, local branch
`codex/local-native-probe-execution`. Package
`omavless 0.0.0.r441.g9bf5781c6c2e-1` was installed through normal polkit
authorization. The executable hash matches its package build-identity record;
21 installed frontend files match the checkout byte-for-byte. No remote writes
or marketplace/main changes were performed.

The native daemon and frontend were installed from this candidate. An explicit
Omarchy shell restart was needed to clear cached frontend JavaScript; it did
not kill either the active requested tunnel core or the owned probe child.
Fresh frontend diagnostics reported coherent native facts and no uncertain
mutation. This is ordinary shell-restart evidence, not fresh-login acceptance.

Only aggregate facts from private fixtures were retained:

| Installed case | Result | Ownership / cleanup / privacy |
| --- | --- | --- |
| Disconnected subscription probe | Completed: 21 rows, 19 DNS-resolved and 19 reachable; two DNS failures | One daemon-owned auxiliary core, zero TUN, no new TCP listeners, exact desired/store preservation, no scratch left |
| Connected cancellation, Routing | Cancelled after auxiliary ownership was observed | Requested core/TUN remained 1/1; auxiliary removed; desired/store preserved |
| Connected subscription probe, Routing | Completed but no positive measurements: one resolved row, zero reachable; remaining 20 DNS-unresolved | Primary/auxiliary attribution, TUN preservation, listener bounds, cleanup and state/store preservation passed; **positive connected measurement remains unproved** |
| Disconnect during connected probe | Explicit disconnect succeeded; invalidated job ended `failed/conflict` | Auxiliary drained, requested core/TUN removed, Routing/disconnected restored; no manual recovery |
| Daemon SIGKILL during disconnected probe | Exact user-unit main process killed while its auxiliary child existed; automatic restart healthy | Old child gone, marked scratch cleaned, desired/store unchanged, zero core/TUN, no new TCP listeners |
| Old operation after daemon restart | `not_found`, as specified by the instance-scoped operation contract | No replay/adoption of the old job |
| Actual installed QML Service → launcher → native job → result parser | Completed: 21 rows, 19 resolved/reachable, `unknown=false`, controls actionable, no pending mutation | Uses the real private fixture internally; output contains only aggregate counts |

The SIGKILL test targeted only `omavless-runtime.service`'s main process, not
the shell, user manager or unrelated processes. Its `KillMode=control-group`
removed the child before successor startup reconciliation. This does not prove
connected crash/reconnect, suspend, physical network transitions or a new login.

### Connected DNS finding

The ordinary installed native HTTPS connection test passed under Routing. This
is `scope=current_route_https`, **not proof of proxy/tunnel egress**; the follow-up
below corrects the earlier interpretation. A separate fresh-cache comparison used the same private hostname
and the configured DoH policy, without printing either. With VPN disconnected,
both Rust and Python's DoH-only reference returned one address, with and without
resolver health filtering. Connected in Routing, both returned zero addresses
in both variants. Thus this observation is **not evidence of a Rust-only DNS
regression**. The Routing HTTPS result cannot exclude a proxy-path failure.
At this stage the connected DoH network/path cause remained unisolated.
No protocol code, host firewall, DNS policy, fallback
resolver or timeout was changed to manufacture a passing result.

Python's unrestricted libc fallback was deliberately excluded from that
comparison; the native resolver's documented bounded/private policy remains in
force. This is DoH-path parity evidence, not a claim that every historical
Python fallback is reproduced. The UI distinguishes unresolved DNS from a
measured unreachable server. Positive connected subscription measurement is
still an explicit acceptance gap under this environment/policy.

### EN/RU rendering and privacy

Six complete affected views were rendered and inspected on the real installed
Quickshell stack: EN/RU running, completed and conflict-failure states. Exact
installed frontend files were copied byte-for-byte to an isolated render
harness with a no-op backend and synthetic profile/subscription metadata; it
could not access the private store, daemon or network. The harness adjusted only
its host anchor, not the product QML. Both locales showed progress, Cancel/Close,
Test, ping sorting, selected-subscription management and result labels. Russian
actions wrap to a second row without overlapping adjacent content; DNS failure,
unavailable and millisecond labels are localized, while synthetic names remain
untranslated. Captures cover all three representative rows without hidden
scrolled content. They are local-only evidence, not a full keyboard/pointer
acceptance of every existing panel state.

Live CLI/QML reports were restricted to public case slugs, states, stable error
codes, counts and booleans; no private IDs, names, endpoints, URI, provider data,
controller path/secret or raw backend errors are included here. Private fixtures
and actual job results were never committed. No manual-recovery-required result
occurred. Final network state was Routing/disconnected, native daemon running,
zero requested/auxiliary Mihomo and zero TUN; plugin enabled.

Remaining: positive connected measurement under a usable configured DoH path,
full human keyboard/pointer acceptance, connected crash/reconciliation, fresh
login and broader package/rollback/R6 retirement gates. This checkpoint restores
a useful native subscription test; it does **not** declare full Python parity,
R5/R6 completion, V0 completion or permission to merge.

## DNS follow-up: transport/fixture isolation

Same installed runtime code `9bf5781c6c2ef20168cb25a25e8a6ae8ec427065`, frontend
`de3b8c88215d31318430cb02078bac4151336684`; no runtime/DNS implementation or
host network configuration changed during this investigation. Remote main was
refreshed and remained `27e2e793f0e14f19f41dce947e06667ca9bf5ec3`.

1. Direct DoH diagnostics distinguished TLS/connect failure from malformed DNS
   replies. The second configured resolver answered public and private A queries
   without VPN, but timed out through the previously selected VPN. A diagnostic
   six-second timeout ended in TLS EOF; changing SNI while retaining the same
   public resolver IP did not recover it. Neither experiment changed production
   timeout/TLS configuration.
2. The owned core's read-only DNS query also failed (HTTP 500 after about five
   seconds). Moving resolution to that API is not an established solution.
   Its behavior was checked against the pinned
   [Mihomo v1.19.30 DNS route implementation](https://github.com/MetaCubeX/mihomo/blob/v1.19.30/hub/route/dns.go).
3. Full VPN/global with the previously selected profile failed the ordinary
   native HTTPS check. Therefore the earlier Routing success was insufficient
   to declare that proxy path healthy.
4. A fresh disconnected batch found 18 reachable candidates. An already-imported
   alternative selected internally from those results passed Full VPN HTTPS
   and public/private configured DoH queries in **the same VM**, with one
   core/TUN. This is positive connected DNS evidence, not a completed connected
   subscription-latency batch. It argues against a VM-wide or universal Rust DNS
   failure; it does not rule out destination-specific VM/host/network behavior.
5. The owner explicitly selected a standalone VLESS fixture for further tests.
   It failed Full VPN HTTPS and the exact core proxy's fixed public HTTPS delay
   test (HTTP 504/timeout). It also failed an isolated no-TUN auxiliary probe
   after its endpoint had been resolved through DoH and pinned to a public IP.
   All three fixed probe rounds ran with normal cleanup. This excludes primary
   TUN routing and lookup of that endpoint as necessary causes of this failure.
6. That requested fixture's Rust/Python rendered proxy mappings were compared
   structurally in private memory and matched. Its server hostname resolved to
   one public address, but a direct TCP connection to that address and the
   configured port timed out after five seconds while VPN was disconnected.

The actionable finding is **requested server/port transport unavailable from
the current network**, not an established DNS parser/config-generation defect.
Server outage, provider/ISP filtering, stale fixture or destination-specific
host/VM network behavior remain possible; RKN blocking is **not proven**.
No resolver substitution, security relaxation, protocol workaround or ceremonial
implementation commit was made. A usable requested endpoint or independent
provider/network check is needed before claiming that fixture works.

The first alternative-profile comparison did not fully restore its original
selection in its `finally` block; observation confirmed disconnected/global,
zero core/TUN and no manual recovery. Before further testing, the owner's newly
specified fixture was selected through the semantic CLI and final restoration
completed: requested fixture selected, Routing/disconnected, core/auxiliary/TUN
0/0/0, native daemon running and plugin enabled. No private store was edited
directly. Local scripts kept real values in memory and printed only booleans,
counts, status/error classes and timings. No private fixture, result, rendered
configuration or screenshot was committed.
