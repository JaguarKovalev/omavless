// SPDX-License-Identifier: MIT
use super::*;
use serde_json::json;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn policy() -> ResolverPolicy {
    ResolverPolicy::from_trusted_configs(None,
        "dns:\n  default-nameserver:\n    - 1.1.1.1\n  direct-nameserver:\n    - https://8.8.8.8/dns-query\n    - https://1.1.1.1/dns-query#PROXY\n").unwrap()
}

fn answer(q: &DnsQuestion, ips: &[IpAddr]) -> Vec<u8> {
    let mut packet = q.wire().to_vec();
    packet[2..4].copy_from_slice(&0x8180u16.to_be_bytes());
    packet[6..8].copy_from_slice(&(ips.len() as u16).to_be_bytes());
    for ip in ips {
        packet.extend([0xc0, 12]);
        packet.extend(
            match ip {
                IpAddr::V4(_) => 1u16,
                IpAddr::V6(_) => 28u16,
            }
            .to_be_bytes(),
        );
        packet.extend(1u16.to_be_bytes());
        packet.extend(0u32.to_be_bytes());
        let octets = match ip {
            IpAddr::V4(v) => v.octets().to_vec(),
            IpAddr::V6(v) => v.octets().to_vec(),
        };
        packet.extend((octets.len() as u16).to_be_bytes());
        packet.extend(octets);
    }
    packet
}

fn ip(text: &str) -> IpAddr {
    text.parse().unwrap()
}

#[test]
fn policy_preserves_priority_annotations_dedup_and_active_precedence() {
    let active = "dns:\n  nameserver:\n    - https://1.1.1.1/dns-query#PROXY\n  proxy-server-nameserver:\n    - https://8.8.8.8/dns-query\n  direct-nameserver:\n    - 'https://9.9.9.9/dns-query'\n    - https://1.1.1.1/dns-query\n";
    let result = ResolverPolicy::from_trusted_configs(
        Some(active),
        "nameserver:\n  - https://8.8.4.4/dns-query\n",
    )
    .unwrap();
    assert_eq!(
        result
            .endpoints
            .iter()
            .map(|e| e.0.host_str().unwrap())
            .collect::<Vec<_>>(),
        ["9.9.9.9", "1.1.1.1", "8.8.8.8"]
    );
}

#[test]
fn policy_falls_back_only_when_no_usable_https_and_never_invents_defaults() {
    assert_eq!(
        ResolverPolicy::from_trusted_configs(None, "dns: {}\n")
            .unwrap()
            .resolver_count(),
        0
    );
    assert_eq!(
        ResolverPolicy::from_trusted_configs(
            Some("nameserver:\n  - udp://1.1.1.1\n"),
            "nameserver:\n  - https://8.8.8.8/dns-query\n"
        )
        .unwrap()
        .resolver_count(),
        1
    );
    for value in [
        "http://1.1.1.1/dns-query",
        "https://user:secret@1.1.1.1/dns-query",
        "https://@1.1.1.1/dns-query",
        "https:1.1.1.1/dns-query",
        "https://127.0.0.1/dns-query",
        "https://1.1.1.1:8443/dns-query",
        "https://[::1]/dns-query",
    ] {
        assert!(DohEndpoint::parse(value).is_none());
    }
}

#[test]
fn policy_bounds_and_bootstrap_same_config() {
    assert!(ResolverPolicy::from_trusted_configs(None, &"x".repeat(MAX_CONFIG_BYTES + 1)).is_err());
    let lines = (1..=9)
        .map(|n| format!("  - https://1.1.1.{n}/dns-query\n"))
        .collect::<String>();
    assert!(ResolverPolicy::from_trusted_configs(None, &format!("nameserver:\n{lines}")).is_err());
    assert_eq!(policy().bootstrap, [ip("1.1.1.1")]);
    let no_bootstrap = ResolverPolicy::from_trusted_configs(
        Some("nameserver:\n  - https://8.8.8.8/dns-query\n"),
        "default-nameserver:\n  - 1.1.1.1\n",
    )
    .unwrap();
    assert!(no_bootstrap.bootstrap.is_empty());
}

#[test]
fn public_ranges_exclude_private_fake_ip_special_and_multicast() {
    for value in [
        "0.1.2.3",
        "10.0.0.1",
        "100.64.0.1",
        "127.0.0.1",
        "169.254.1.2",
        "172.31.255.255",
        "192.0.0.9",
        "192.0.2.1",
        "192.88.99.1",
        "192.168.0.1",
        "198.18.0.1",
        "198.19.255.255",
        "198.51.100.1",
        "203.0.113.1",
        "224.0.0.1",
        "240.0.0.1",
        "255.255.255.255",
        "::",
        "::1",
        "::ffff:1.1.1.1",
        "64:ff9b::808:808",
        "fc00::1",
        "fe80::1",
        "ff02::1",
        "2001:db8::1",
        "2002::1",
        "3fff::1",
    ] {
        assert!(
            !public_address(ip(value)),
            "public policy accepted reserved address"
        );
    }
    for value in [
        "1.1.1.1",
        "8.8.8.8",
        "77.88.8.8",
        "100.63.255.255",
        "100.128.0.0",
        "172.15.255.255",
        "172.32.0.0",
        "198.17.255.255",
        "198.20.0.0",
        "2606:4700:4700::1111",
        "2001:4860:4860::8888",
    ] {
        assert!(
            public_address(ip(value)),
            "public policy rejected ordinary address"
        );
    }
}

#[test]
fn questions_canonicalize_idna_and_enforce_labels() {
    assert_eq!(
        DnsQuestion::new("EXAMPLE.COM.", 7, 1).unwrap().name,
        "example.com"
    );
    assert_eq!(
        DnsQuestion::new("пример.рф", 7, 28).unwrap().name,
        "xn--e1afmkfd.xn--p1ai"
    );
    for value in [
        "",
        "a..b",
        "-host.example",
        "host_.example",
        "name/path",
        "secret@host",
        "127.0.0.1",
    ] {
        assert!(DnsQuestion::new(value, 1, 1).is_err());
    }
    assert!(DnsQuestion::new(&format!("{}.example", "x".repeat(64)), 1, 1).is_err());
    assert!(DnsQuestion::new("example.com", 1, 15).is_err());
}

#[test]
fn response_binds_transaction_flags_question_type_and_name() {
    let q = DnsQuestion::new("example.com", 3, 1).unwrap();
    let baseline = answer(&q, &[ip("1.1.1.1")]);
    assert_eq!(q.addresses(&baseline).unwrap(), [ip("1.1.1.1")]);
    for index in [0, 3, 4, 13] {
        let mut bad = baseline.clone();
        bad[index] ^= 1;
        assert!(q.addresses(&bad).is_err());
    }
    let other = DnsQuestion::new("example.com", 3, 28).unwrap();
    assert!(q.addresses(&answer(&other, &[])).is_err());
    let mut truncated = baseline.clone();
    truncated[2] |= 2;
    assert!(q.addresses(&truncated).is_err());
    let mut request = baseline;
    request[2] &= 0x7f;
    assert!(q.addresses(&request).is_err());
}

#[test]
#[ignore = "opt-in fixed public HTTPS DoH network check; no private profile or core"]
fn installed_public_https_doh_without_system_resolver() {
    let policy = ResolverPolicy::from_trusted_configs(
        None,
        "nameserver:\n  - https://1.1.1.1/dns-query\n  - https://8.8.8.8/dns-query\n",
    )
    .unwrap();
    let transport = HttpsDohTransport::for_policy(&policy);
    let mut resolver = ProbeResolver::new(policy, transport);
    let result = resolver.resolve("example.com", HOST_BUDGET, &|| false);
    assert!(
        result.is_ok_and(|addresses| !addresses.is_empty()),
        "Public DoH check unavailable"
    );
}

#[test]
fn response_is_bounded_and_never_accepts_trailing_or_malformed_bytes() {
    let q = DnsQuestion::new("example.com", 3, 1).unwrap();
    let baseline = answer(&q, &[ip("1.1.1.1")]);
    for n in 0..baseline.len() {
        assert!(q.addresses(&baseline[..n]).is_err());
    }
    let mut trailing = baseline;
    trailing.push(0);
    assert!(q.addresses(&trailing).is_err());
    assert!(q.addresses(&vec![0; MAX_DNS_BYTES + 1]).is_err());
    let mut count = answer(&q, &[]);
    count[6..8].copy_from_slice(&257u16.to_be_bytes());
    assert!(q.addresses(&count).is_err());
}

#[test]
fn response_rejects_cyclic_forward_or_reserved_name_pointers() {
    let q = DnsQuestion::new("example.com", 3, 1).unwrap();
    let at = q.wire().len();
    for bytes in [
        [0xc0, at as u8],
        [0xc0, (at + 2) as u8],
        [0xff, 0xff],
        [0x80, 12],
        [0x40, 12],
    ] {
        let mut packet = answer(&q, &[ip("1.1.1.1")]);
        packet[at..at + 2].copy_from_slice(&bytes);
        assert!(q.addresses(&packet).is_err());
    }
}

#[test]
fn cname_chain_allows_related_additional_but_not_unrelated_addresses() {
    let q = DnsQuestion::new("example.com", 4, 1).unwrap();
    let mut packet = answer(&q, &[]);
    packet[6..8].copy_from_slice(&1u16.to_be_bytes());
    packet[10..12].copy_from_slice(&2u16.to_be_bytes());
    packet.extend([0xc0, 12, 0, 5, 0, 1, 0, 0, 0, 0]);
    let target = b"\x05alias\x07example\x03com\0";
    packet.extend((target.len() as u16).to_be_bytes());
    packet.extend(target);
    for owner in [target.as_slice(), b"\x05other\x07example\x03com\0"] {
        packet.extend(owner);
        packet.extend([0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 1, 1, 1, 1]);
    }
    assert_eq!(q.addresses(&packet).unwrap(), [ip("1.1.1.1")]);
    let mut unrelated = answer(&q, &[ip("1.1.1.1")]);
    let at = q.wire().len();
    unrelated.splice(at..at + 2, b"\x05other\x07example\x03com\0".iter().copied());
    assert!(q.addresses(&unrelated).unwrap().is_empty());
}

#[test]
fn response_filters_deduplicates_and_caps_four() {
    let q = DnsQuestion::new("example.com", 4, 1).unwrap();
    let addresses = [
        "198.18.0.1",
        "1.1.1.1",
        "1.1.1.1",
        "8.8.8.8",
        "9.9.9.9",
        "77.88.8.8",
        "8.8.4.4",
    ]
    .map(ip);
    assert_eq!(
        q.addresses(&answer(&q, &addresses)).unwrap(),
        ["1.1.1.1", "8.8.8.8", "9.9.9.9", "77.88.8.8"].map(ip)
    );
    let q6 = DnsQuestion::new("example.com", 4, 28).unwrap();
    assert_eq!(
        q6.addresses(&answer(
            &q6,
            &[ip("2001:db8::1"), ip("2606:4700:4700::1111")]
        ))
        .unwrap(),
        [ip("2606:4700:4700::1111")]
    );
}

struct Scripted {
    calls: Vec<(String, u16)>,
    fail_first: bool,
    fail_all: bool,
}
impl DohTransport for Scripted {
    fn query(
        &mut self,
        endpoint: &DohEndpoint,
        question: &DnsQuestion,
        _deadline: Instant,
        _cancelled: &dyn Fn() -> bool,
    ) -> Result<Vec<u8>, ResolveError> {
        self.calls
            .push((endpoint.0.host_str().unwrap().to_owned(), question.kind));
        if self.fail_all || (self.fail_first && endpoint.0.host_str() == Some("8.8.8.8")) {
            return Err(ResolveError::Unavailable);
        }
        Ok(answer(
            question,
            &[if question.kind == 1 {
                ip("1.1.1.1")
            } else {
                ip("2606:4700:4700::1111")
            }],
        ))
    }
}
fn scripted(fail_first: bool, fail_all: bool) -> ProbeResolver<Scripted> {
    ProbeResolver::new(
        policy(),
        Scripted {
            calls: Vec::new(),
            fail_first,
            fail_all,
        },
    )
}

#[test]
fn resolves_both_families_then_stops_at_first_working_resolver() {
    let mut resolver = scripted(false, false);
    assert_eq!(
        resolver
            .resolve("example.com", HOST_BUDGET, &|| false)
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        resolver.transport.calls,
        [("8.8.8.8".into(), 1), ("8.8.8.8".into(), 28)]
    );
    let mut fallback = scripted(true, false);
    assert_eq!(
        fallback
            .resolve("example.com", HOST_BUDGET, &|| false)
            .unwrap()
            .len(),
        2
    );
    assert_eq!(fallback.transport.calls.len(), 4);
}

#[test]
fn all_failed_means_unresolved_and_no_ambient_dns_fallback() {
    let mut resolver = scripted(false, true);
    assert!(
        resolver
            .resolve("example.com", HOST_BUDGET, &|| false)
            .unwrap()
            .is_empty()
    );
    assert_eq!(resolver.transport.calls.len(), 4);
}

#[test]
fn literal_addresses_do_not_call_transport() {
    let mut resolver = scripted(false, true);
    assert_eq!(
        resolver.resolve("1.1.1.1", HOST_BUDGET, &|| false).unwrap(),
        [ip("1.1.1.1")]
    );
    assert!(
        resolver
            .resolve("198.18.0.1", HOST_BUDGET, &|| false)
            .unwrap()
            .is_empty()
    );
    assert!(resolver.transport.calls.is_empty());
}

#[test]
fn cancellation_and_zero_budget_have_no_io() {
    let mut resolver = scripted(false, false);
    assert_eq!(
        resolver.resolve("example.com", HOST_BUDGET, &|| true),
        Err(ResolveError::Cancelled)
    );
    assert_eq!(
        resolver.resolve("example.com", Duration::ZERO, &|| false),
        Err(ResolveError::Timeout)
    );
    assert!(resolver.transport.calls.is_empty());
    let p = policy();
    let mut transport = HttpsDohTransport::for_policy(&p);
    assert!(matches!(
        transport.query(
            &p.endpoints[0],
            &question("example.com", 1).unwrap(),
            Instant::now(),
            &|| true
        ),
        Err(ResolveError::Cancelled)
    ));
}

#[test]
fn working_filter_retains_order_or_all_configured_when_none_work() {
    let mut partial = scripted(true, false);
    partial.filter_working(HOST_BUDGET, &|| false).unwrap();
    assert_eq!(partial.policy.resolver_count(), 1);
    assert_eq!(partial.policy.endpoints[0].0.host_str(), Some("1.1.1.1"));
    let mut all_failed = scripted(false, true);
    all_failed.filter_working(HOST_BUDGET, &|| false).unwrap();
    assert_eq!(all_failed.policy.resolver_count(), 2);
}

#[test]
fn errors_and_pinned_debug_never_contain_private_input() {
    let error = DnsQuestion::new("https://private.invalid/password?key=secret", 1, 1)
        .err()
        .unwrap();
    assert_eq!(error.to_string(), "Probe DNS name is invalid");
    let resolver = PinnedResolver {
        host: "private.invalid".into(),
        addresses: vec![ip("1.1.1.1")],
    };
    assert_eq!(format!("{resolver:?}"), "PinnedProbeResolver");
    for error in [
        ResolveError::Policy,
        ResolveError::InvalidName,
        ResolveError::InvalidResponse,
        ResolveError::Unavailable,
        ResolveError::Timeout,
        ResolveError::Cancelled,
    ] {
        assert!(error.to_string().len() < 64);
    }
}

#[test]
fn builtin_templates_have_bounded_https_and_public_bootstrap() {
    for template in [
        include_str!("../../../templates/default.yaml"),
        include_str!("../../../templates/china.yaml"),
        include_str!("../../../templates/iran.yaml"),
    ] {
        let p = ResolverPolicy::from_trusted_configs(None, template).unwrap();
        assert!(p.resolver_count() > 0);
        assert_eq!(p.bootstrap.len(), 2);
    }
}

#[test]
fn actual_python_dns_oracle_matches_valid_packets_and_resolver_order() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut cases = Vec::new();
    for (host, kind, addresses) in [
        ("example.com", 1, vec![ip("1.1.1.1"), ip("198.18.0.1")]),
        (
            "example.com",
            28,
            vec![ip("2606:4700:4700::1111"), ip("2001:db8::1")],
        ),
        ("EXAMPLE.COM.", 1, vec![]),
        ("пример.рф", 1, vec![ip("8.8.8.8")]),
    ] {
        let q = DnsQuestion::new(host, 7, kind).unwrap();
        let packet = answer(&q, &addresses);
        // Python preserves ASCII case in its wire question; normalize the input
        // exactly once before comparing semantic qnames and answer projection.
        cases.push(json!({"host":q.name,"kind":kind,"question":q.wire(),"packet":packet,"addresses":q.addresses(&packet).unwrap().iter().map(ToString::to_string).collect::<Vec<_>>() }));
    }
    let mut child = Command::new("python3")
        .arg(root.join("tools/probe_resolver_parity.py"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(
            serde_json::to_string(&json!({"cases":cases}))
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "Synthetic DNS oracle failed");
    assert_eq!(
        output.stdout,
        b"PASS 4 DNS cases; resolver order; health filter; address fallback\n"
    );
}
