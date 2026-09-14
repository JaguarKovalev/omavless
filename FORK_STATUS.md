# JaguarKovalev legacy fork status

## Known-good recovery point

The tested working snapshot is commit `f6b868149eeb26b42ceac8ee9e8bfd2c044cd5bc` (OmaVLESS `0.7.4`).

It is preserved on the branch:

- `archive-0.7.4-known-good`

Observed on Omarchy/Arch after a reboot:

- `omavless.service` active without an authentication prompt;
- TUN interface `Meta` present with `198.18.0.1/30`;
- Full VPN public traffic exits through the selected VLESS/Reality server;
- `PROXY -> selected profile` and `GLOBAL -> PROXY` retained;
- no `netlink receive: file exists` TUN startup error;
- no iptables `Permission denied (you must be root)` fallback failure.

## Why 0.7.4 strips route-exclude-address at runtime

On the tested current Arch/Omarchy kernel, Mihomo/sing-tun native nftables auto-redirect fails while creating the nftables interval sets used by `route-exclude-address`, returning `EEXIST` (`netlink receive: file exists`).

Using `DISABLE_NFTABLES=true` is not suitable for this rootless user-service design because Mihomo then launches `/usr/bin/iptables` as a child process. The child does not inherit Mihomo's file capabilities and fails with a root-permission error.

The maintained fork therefore keeps native nftables and removes only the `route-exclude-address` block from the generated runtime config. The source routing templates remain unchanged.

## 0.7.5 hardening branch

`work-0.7.5-hardening` is for regression tests and nonessential cleanup. Do not move the known-good archive branch while testing new changes.

Current hardening goals:

1. regression coverage for selector readiness, runtime TUN config and TUN readiness;
2. no successful UI state unless the `Meta` interface really exists;
3. ICMP latency probing disabled by default to avoid noisy timeout logs on networks where the configured probe target is unsuitable;
4. evaluate a safe LAN-bypass strategy separately without changing the known-good 0.7.4 snapshot.
