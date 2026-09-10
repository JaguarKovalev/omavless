# R5 native subscription-probe DNS boundary

This local checkpoint adds `native_probe_resolver` only. It does not register
an operation, read a private store, launch Mihomo, touch a tunnel, or install
anything. It is a prerequisite of the owned subscription-latency executor;
Python remains the reference and cannot be retired on this evidence.

## API and ownership

The admitted owner supplies already trusted active-config/template text to
`ResolverPolicy::from_trusted_configs(active, template)`. No YAML or arbitrary
resolver URL is accepted through IPC. As in Python, the first config containing
usable HTTPS resolvers wins; resolver order is direct-nameserver, then
proxy-server-nameserver, then nameserver. Annotations are removed and duplicate
URLs are suppressed. No default resolver is invented. An empty policy can
still resolve an already-public literal IP without network I/O.

Construct `HttpsDohTransport::for_policy(&policy)` and then
`ProbeResolver::new(policy, transport)` once per admitted batch. The optional
`filter_working` performs the reference example.com/A health filter, retaining
the configured list if all checks fail within the budget. Call `resolve` with
the private profile host, remaining job duration and a cancellation predicate.
Both A and AAAA are requested in order; the first resolver with useful answers
wins. Empty output means unresolved, not measured provider unreachability.
Resolver errors contain only fixed English classifications. Endpoint policy,
DNS questions, transport caches and resolver objects cannot be debug-formatted.

The executor must apply ownership/cancellation/result-publication fences and
map positions to internal record IDs. This helper never publishes those IDs.

## Network and parser bounds

- 2 MiB per trusted config, eight DoH URLs, 2048 bytes per URL, four bootstrap
  addresses, four resolved public addresses per profile.
- Twelve seconds maximum per host/filter, shortened by the enclosing job.
  Each HTTPS exchange is globally bounded at two seconds; synchronous
  cancellation may wait for that bounded exchange, never a detached thread.
- DNS responses: 64 KiB; HTTPS headers: 8 KiB; at most 256 RRs; compressed-name
  traversal: 128 steps and 253-byte names; CNAME chain: 16 names.
- Strict transaction, response flags, original qname/type/class and complete
  frame binding; malformed compression, truncation, trailing data and unrelated
  additional-address records are rejected/ignored appropriately. Only addresses
  owned by the query or its bounded CNAME chain can be returned.
- Globally routable unicast only: no loopback, private, link-local, multicast,
  documentation, transition/special-purpose or Mihomo fake-IP addresses.
- Fixed POST with DNS-message headers, verified TLS, no HTTP redirects, no
  ambient proxy, cookies or credentials. The original DoH authority remains the
  TLS identity while the transport connects only to validated pinned addresses.
- A custom ureq resolver prevents libc `getaddrinfo` and lookup-worker escape.
  Numeric DoH endpoints need no bootstrap. Named endpoints use only public
  numeric default-nameserver entries from the *same* trusted config, through
  connected UDP sockets with random transaction IDs, strict DNS binding and
  cancellation-aware receive polling. Only the public DoH hostname goes through
  bootstrap; private profile names are sent only inside TLS-protected DoH.

## Deliberate reference narrowings

Python accepts unbound address RRs, arbitrary HTTPS ports and certain
special-purpose globally marked addresses, follows urllib redirects/proxies,
and finally calls unbounded libc DNS. Those behaviors are not copied. Native
DoH requires port 443 and public pinned endpoints, rejects misleading userinfo
and backslash URL forms, and omits libc fallback. Nonstandard trusted policies
may therefore yield unresolved profiles instead of silently escaping the DNS
policy. Named DoH requires configured numeric bootstrap; absent bootstrap is
not replaced with hard-coded third-party defaults.

The ordinary public IPv4/IPv6 contract, resolver order, annotation/deduplication,
health-filter fallback, A/AAAA fallback and four-address cap are preserved.
Private/special DNS deployments and standalone scoped IPv6 are not covered by
this internet-server latency helper. This does not change connection DNS.

## Evidence

18 deterministic tests pass, including a four-packet differential against actual
Python `dns_question`/`parse_dns_addresses` and effect-isolated checks of actual
resolver-policy selection, health-filter fallback and address-query order.
Synthetic packets cover direct A/AAAA, IDNA, CNAME/additional owner binding,
compression cycles, reserved/forward pointers, mismatched IDs/questions,
truncation, oversized packets/counts, fake-IP/private exclusions and dedup/caps.

The fixed public HTTPS DoH opt-in passed once on Try Omarchy ARM64, using only
example.com and generic public DoH endpoints, without accessing private
fixtures, launching a core or modifying the installed runtime. This is transport
evidence, not private-provider or whole-executor acceptance.

Focused compilation/tests and strict clippy were performed directly against
the new module using already-built locked dependency rlibs to avoid another
multi-gigabyte runtime target in the space-constrained VM. Root integration
must still run the actual Cargo runtime gate after combining this change.
No new dependency or Cargo.lock change is needed. The ureq unversioned custom
resolver API is pinned by the existing lockfile and must be retested on upgrades.
