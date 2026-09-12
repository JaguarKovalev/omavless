# R6 network outage diagnosis — 2026-09-13

Try Omarchy ARM64 VM. Local diagnosis only; no publication or R6 completion.
Installed runtime source: `7b75b747883d66a05f1f2b321d42f194c6040b5c`.
Package: `omavless 0.0.0.r492.g7b75b747883d-1`.
Executable SHA-256:
`fe04fba32d4e135d56e929350cd0296d3f89168e06d6ebba9a6de089aec4b786`.
Source at investigation start: `0356bc0` on
`codex/local-native-probe-execution`. Remote main remains `27e2e793`.

## Question and chronology

The owner reports a working VPN earlier, then a post-reboot outage: enabling
VPN prevents VM networking and disabling restores it. Do not characterize this
as a proven defect present since the Rust port began. Earlier recorded HTTPS
timeouts and this observation do not establish a common cause or exact onset.

The preceding connected diagnostic also cut the agent's transport. The owner
disconnected to recover. This pass therefore leaves the installed VPN
disconnected and tests the existing outbound profile in a temporary core with
TUN disabled. No system route, resolver setting, service, startup preference,
polkit policy or installed configuration is changed.

## Evidence

| Check | Observed result |
| --- | --- |
| Installed binary vs pre-reboot checkpoint | Identical SHA-256 |
| Earlier `46b4413` to current `7b75b74`, selected network paths | No changes in `core.rs`, domain `config.rs` or bundled default template; `native_host.rs` only adds the read-only support-facts method. This is not an exhaustive regression exclusion. |
| Selected fixture | Last selection matches the recorded login fixture and the owner's previously requested fixture |
| Generated endpoint | Host and port match that fixture's stored URI; values not published |
| Direct fixed public HTTPS with VPN off | HTTP 200 |
| Server system DNS | Resolves one public IPv4 address, outside fake-IP `198.18.0.0/15`; no private-address answer |
| Route to resolved server with VPN off | Route exists with a gateway |
| Plain TCP connection to that server/port with VPN off | Connect timeout; no VPN protocol authentication attempted |
| Same rendered VLESS outbound through temporary loopback SOCKS, no TUN | Both remote-DNS and pre-resolved-target HTTPS fail; curl exit 35 after about five seconds |
| Temporary core log | Two `dial tcp` / `i/o timeout` failures; timeout occurs before successful outbound TLS/protocol establishment |
| Cleanup | Temporary core terminated/reaped; installed runtime observed disconnected, Mihomo/TUN/auxiliary zero, manual recovery false |
| Production configuration | Byte-identical before/after isolated diagnostic |

The temporary configuration copies the rendered outbound without changing its
credentials or transport fields. It uses a private temporary data directory,
loopback-only mixed port, fixed PROXY/GLOBAL selectors and no external controller,
TUN, automatic routes or rule-provider downloads. Existing DNS upstream settings
are retained, but rule-set references in fake-IP filters are removed **only in
this isolated fixture**, since no routing providers are supplied. This tests
outbound connectivity, not full production routing-policy parity.

An initial isolated harness attempt correctly refused a missing rule-set
reference before any proxy probe. Correcting that temporary fixture is not a
production fix. A preliminary loose log substring counter matched `403` without
proving an HTTP status; the corrected classifier found no confirmed HTTP 403.
Do not report an upstream HTTP rejection from that initial counter.

## Conclusion and limits

The current failure reproduces with the installed VPN disconnected and without
Rust lifecycle/TUN involvement: the selected server/port cannot establish TCP
from this VM's current network. This is strong evidence against a failure
confined to autostart, system DNS lookup or TUN setup. It does **not** distinguish
server outage, filtering along the path, an incorrect public DNS answer, or a
changed provider endpoint. A public address is not proof of DNS authenticity.
Nor does it prove the full Rust networking path has no independent defects.

Do not change production protocol code, DNS, OS permissions or deadlines to
make this fixture pass. The useful next comparison is the same private fixture
on an independent known-working client/network, or provider-side availability
confirmation. Do not publish its endpoint or ask for credentials in chat.

The temporary no-TUN test cannot close fresh connected-login acceptance. Keep
Last/pinned and R6 completion unclaimed. Startup remains Last/Routing/enabled;
restoration to the original Off preference is still outstanding. No connected
VPN transition or authentication dialog was initiated by this pass.

## Privacy and persistence

Diagnostic scripts and private temporary fixtures/logs remain outside Git;
temporary directories are mode 0700 and generated config/log files mode 0600.
Only boolean/count/classification evidence is emitted; no profile names, IDs,
hosts, addresses, credentials, raw controller/core errors or subscription data
are in this report. No implementation was changed or pushed. Existing static
acceptance remains scoped to its original tested source; it was not rerun for
this documentation-only finding.
