// SPDX-License-Identifier: MIT

//! Private, bounded subscription-probe DNS. No system resolver or background
//! lookup threads are used. The caller supplies already trusted config text;
//! this module neither reads the private store nor changes runtime ownership.

use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};
use ureq::Agent;
use ureq::tls::{RootCerts, TlsConfig};
use ureq::unversioned::resolver::{ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::{DefaultConnector, NextTimeout};
use url::{Host, Url};

pub const MAX_RESOLVERS: usize = 8;
pub const MAX_ADDRESSES: usize = 4;
pub const MAX_DNS_BYTES: usize = 64 * 1024;
pub const HOST_BUDGET: Duration = Duration::from_secs(12);
pub const EXCHANGE_BUDGET: Duration = Duration::from_secs(2);
const MAX_CONFIG_BYTES: usize = 2 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveError {
    Policy,
    InvalidName,
    InvalidResponse,
    Unavailable,
    Timeout,
    Cancelled,
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Policy => "Probe DNS policy is unavailable",
            Self::InvalidName => "Probe DNS name is invalid",
            Self::InvalidResponse => "Probe DNS response is invalid",
            Self::Unavailable => "Probe DNS is unavailable",
            Self::Timeout => "Probe DNS timed out",
            Self::Cancelled => "Probe DNS was cancelled",
        })
    }
}
impl std::error::Error for ResolveError {}

/// Credential-bearing policy URL: deliberately no Debug/Serialize.
#[derive(Clone)]
pub struct DohEndpoint(Url);

impl DohEndpoint {
    fn parse(value: &str) -> Option<Self> {
        if value.len() > 2048 || value.bytes().any(|b| b.is_ascii_control()) {
            return None;
        }
        let (_, authority) = value.split_once("://")?;
        if authority.split(['/', '?', '#']).next()?.contains('@') || value.contains('\\') {
            return None;
        }
        let url = Url::parse(value.split('#').next()?).ok()?;
        if url.scheme() != "https"
            || !url.username().is_empty()
            || url.password().is_some()
            || url.host().is_none()
            || url.port_or_known_default() != Some(443)
        {
            return None;
        }
        match url.host()? {
            Host::Ipv4(ip) if !public_address(ip.into()) => return None,
            Host::Ipv6(ip) if !public_address(ip.into()) => return None,
            Host::Domain(name) => {
                canonical_name(name).ok()?;
            }
            _ => {}
        }
        Some(Self(url))
    }
}

/// No defaults are invented. First config with usable HTTPS policy wins, just
/// like the reference; bootstrap is taken from that same selected config.
pub struct ResolverPolicy {
    endpoints: Vec<DohEndpoint>,
    bootstrap: Vec<IpAddr>,
}

impl ResolverPolicy {
    pub fn from_trusted_configs(
        active: Option<&str>,
        template: &str,
    ) -> Result<Self, ResolveError> {
        for text in active.into_iter().chain(std::iter::once(template)) {
            if text.len() > MAX_CONFIG_BYTES || text.contains('\0') {
                return Err(ResolveError::Policy);
            }
            let mut endpoints: Vec<DohEndpoint> = Vec::new();
            for key in ["direct-nameserver", "proxy-server-nameserver", "nameserver"] {
                for item in sequence(text, key)? {
                    if let Some(endpoint) = DohEndpoint::parse(item)
                        && !endpoints.iter().any(|old| old.0 == endpoint.0)
                    {
                        if endpoints.len() == MAX_RESOLVERS {
                            return Err(ResolveError::Policy);
                        }
                        endpoints.push(endpoint);
                    }
                }
            }
            if !endpoints.is_empty() {
                let mut bootstrap = Vec::new();
                for item in sequence(text, "default-nameserver")? {
                    if let Ok(ip) = item.parse::<IpAddr>()
                        && public_address(ip)
                        && !bootstrap.contains(&ip)
                    {
                        if bootstrap.len() == MAX_ADDRESSES {
                            return Err(ResolveError::Policy);
                        }
                        bootstrap.push(ip);
                    }
                }
                return Ok(Self {
                    endpoints,
                    bootstrap,
                });
            }
        }
        Ok(Self {
            endpoints: Vec::new(),
            bootstrap: Vec::new(),
        })
    }

    #[must_use]
    pub fn resolver_count(&self) -> usize {
        self.endpoints.len()
    }
}

fn sequence<'a>(text: &'a str, key: &str) -> Result<Vec<&'a str>, ResolveError> {
    let mut indent = None;
    let mut values = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let depth = line.len() - line.trim_start().len();
        if indent.is_some_and(|old| depth <= old) {
            indent = None;
        }
        if let Some((name, rest)) = trimmed.split_once(':')
            && name.trim() == key
            && rest.trim().is_empty()
        {
            indent = Some(depth);
            continue;
        }
        if indent.is_some() {
            let Some(item) = trimmed.strip_prefix('-') else {
                return Err(ResolveError::Policy);
            };
            values.push(item.trim().trim_matches(['\'', '"']));
            if values.len() > 64 {
                return Err(ResolveError::Policy);
            }
        }
    }
    Ok(values)
}

/// Conservative globally routable unicast policy. Special-purpose transition,
/// benchmarking, documentation and fake-IP ranges are deliberately excluded.
#[must_use]
pub fn public_address(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            let n = u32::from(ip);
            let blocked = [
                (0x00000000, 8),
                (0x0a000000, 8),
                (0x64400000, 10),
                (0x7f000000, 8),
                (0xa9fe0000, 16),
                (0xac100000, 12),
                (0xc0000000, 24),
                (0xc0000200, 24),
                (0xc0586300, 24),
                (0xc0a80000, 16),
                (0xc6120000, 15),
                (0xc6336400, 24),
                (0xcb007100, 24),
                (0xe0000000, 4),
                (0xf0000000, 4),
            ];
            !blocked
                .into_iter()
                .any(|(base, prefix)| n >> (32 - prefix) == base >> (32 - prefix))
        }
        IpAddr::V6(ip) => {
            let s = ip.segments();
            s[0] & 0xe000 == 0x2000
                && !(s[0] == 0x2001 && (s[1] < 0x200 || s[1] == 0xdb8))
                && s[0] != 0x2002
                && !(s[0] == 0x3fff && s[1] & 0xf000 == 0)
        }
    }
}

fn canonical_name(host: &str) -> Result<String, ResolveError> {
    if host.len() > 1024 {
        return Err(ResolveError::InvalidName);
    }
    let Host::Domain(name) =
        Host::parse(host.trim_end_matches('.')).map_err(|_| ResolveError::InvalidName)?
    else {
        return Err(ResolveError::InvalidName);
    };
    if name.is_empty()
        || name.len() > 253
        || name.split('.').any(|label| {
            label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
    {
        return Err(ResolveError::InvalidName);
    }
    Ok(name)
}

/// Private qname remains unformatted, including parse failures.
pub struct DnsQuestion {
    id: u16,
    name: String,
    kind: u16,
    bytes: Vec<u8>,
}

impl DnsQuestion {
    pub fn new(host: &str, id: u16, kind: u16) -> Result<Self, ResolveError> {
        if !matches!(kind, 1 | 28) {
            return Err(ResolveError::InvalidName);
        }
        let name = canonical_name(host)?;
        let mut bytes = Vec::with_capacity(272);
        for word in [id, 0x0100, 1, 0, 0, 0] {
            bytes.extend(word.to_be_bytes());
        }
        for label in name.split('.') {
            bytes.push(label.len() as u8);
            bytes.extend(label.as_bytes());
        }
        bytes.push(0);
        bytes.extend(kind.to_be_bytes());
        bytes.extend(1u16.to_be_bytes());
        Ok(Self {
            id,
            name,
            kind,
            bytes,
        })
    }

    #[must_use]
    pub fn wire(&self) -> &[u8] {
        &self.bytes
    }

    pub fn addresses(&self, packet: &[u8]) -> Result<Vec<IpAddr>, ResolveError> {
        if !(12..=MAX_DNS_BYTES).contains(&packet.len()) {
            return Err(ResolveError::InvalidResponse);
        }
        let word = |at| read_word(packet, at);
        let flags = word(2)?;
        if word(0)? != self.id || flags & 0xf80f != 0x8000 || flags & 0x0200 != 0 || word(4)? != 1 {
            return Err(ResolveError::InvalidResponse);
        }
        let (name, mut at) = read_name(packet, 12)?;
        if name != self.name || word(at)? != self.kind || word(at + 2)? != 1 {
            return Err(ResolveError::InvalidResponse);
        }
        at += 4;
        let count = usize::from(word(6)?) + usize::from(word(8)?) + usize::from(word(10)?);
        if count > 256 {
            return Err(ResolveError::InvalidResponse);
        }
        let mut aliases: BTreeMap<String, String> = BTreeMap::new();
        let mut addresses = Vec::new();
        for _ in 0..count {
            let (owner, next) = read_name(packet, at)?;
            at = next;
            let kind = word(at)?;
            let class = word(at + 2)?;
            let length = usize::from(word(at + 8)?);
            at += 10;
            let value = packet
                .get(at..at + length)
                .ok_or(ResolveError::InvalidResponse)?;
            if class == 1 && kind == 5 {
                let (target, end) = read_name(packet, at)?;
                if end != at + length {
                    return Err(ResolveError::InvalidResponse);
                }
                if aliases
                    .insert(owner.clone(), target.clone())
                    .is_some_and(|old| old != target)
                {
                    return Err(ResolveError::InvalidResponse);
                }
            } else if class == 1 && kind == self.kind {
                let ip = match (kind, value.len()) {
                    (1, 4) => IpAddr::V4(Ipv4Addr::new(value[0], value[1], value[2], value[3])),
                    (28, 16) => IpAddr::V6(Ipv6Addr::from(
                        <[u8; 16]>::try_from(value).map_err(|_| ResolveError::InvalidResponse)?,
                    )),
                    _ => return Err(ResolveError::InvalidResponse),
                };
                if public_address(ip) {
                    addresses.push((owner, ip));
                }
            }
            at += length;
        }
        if at != packet.len() {
            return Err(ResolveError::InvalidResponse);
        }
        let mut chain = vec![self.name.clone()];
        while let Some(next) = aliases.get(chain.last().ok_or(ResolveError::InvalidResponse)?) {
            if chain.len() == 16 || chain.contains(next) {
                return Err(ResolveError::InvalidResponse);
            }
            chain.push(next.clone());
        }
        let mut result = Vec::new();
        for (name, ip) in addresses {
            if chain.contains(&name) && !result.contains(&ip) && result.len() < MAX_ADDRESSES {
                result.push(ip);
            }
        }
        Ok(result)
    }
}

fn read_word(packet: &[u8], at: usize) -> Result<u16, ResolveError> {
    let bytes = packet
        .get(at..at + 2)
        .ok_or(ResolveError::InvalidResponse)?;
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn read_name(packet: &[u8], mut at: usize) -> Result<(String, usize), ResolveError> {
    let mut end = None;
    let mut labels = Vec::new();
    let mut seen = Vec::new();
    let mut size = 0;
    for _ in 0..128 {
        if seen.contains(&at) {
            return Err(ResolveError::InvalidResponse);
        }
        seen.push(at);
        let length = *packet.get(at).ok_or(ResolveError::InvalidResponse)?;
        if length & 0xc0 == 0xc0 {
            let pointer = usize::from(read_word(packet, at)? & 0x3fff);
            if pointer >= at {
                return Err(ResolveError::InvalidResponse);
            }
            end.get_or_insert(at + 2);
            at = pointer;
            continue;
        }
        at += 1;
        if length == 0 {
            return Ok((labels.join("."), end.unwrap_or(at)));
        }
        if length > 63 {
            return Err(ResolveError::InvalidResponse);
        }
        let value = packet
            .get(at..at + usize::from(length))
            .ok_or(ResolveError::InvalidResponse)?;
        if !value
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || *b == b'-')
        {
            return Err(ResolveError::InvalidResponse);
        }
        size += usize::from(length) + 1;
        if size > 254 {
            return Err(ResolveError::InvalidResponse);
        }
        labels.push(
            String::from_utf8(value.to_ascii_lowercase())
                .map_err(|_| ResolveError::InvalidResponse)?,
        );
        at += usize::from(length);
    }
    Err(ResolveError::InvalidResponse)
}

fn remaining(deadline: Instant, cancelled: &dyn Fn() -> bool) -> Result<Duration, ResolveError> {
    remaining_with_clock(deadline, cancelled, &Instant::now)
}

fn remaining_with_clock(
    deadline: Instant,
    cancelled: &dyn Fn() -> bool,
    now: &dyn Fn() -> Instant,
) -> Result<Duration, ResolveError> {
    if cancelled() {
        return Err(ResolveError::Cancelled);
    }
    deadline
        .checked_duration_since(now())
        .filter(|d| !d.is_zero())
        .ok_or(ResolveError::Timeout)
}

fn question(host: &str, kind: u16) -> Result<DnsQuestion, ResolveError> {
    let mut random = [0; 2];
    File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut random))
        .map_err(|_| ResolveError::Unavailable)?;
    DnsQuestion::new(host, u16::from_be_bytes(random), kind)
}

/// Implementations must finish within deadline and never format private input.
pub trait DohTransport {
    fn query(
        &mut self,
        endpoint: &DohEndpoint,
        question: &DnsQuestion,
        deadline: Instant,
        cancelled: &dyn Fn() -> bool,
    ) -> Result<Vec<u8>, ResolveError>;
}

pub struct ProbeResolver<T> {
    policy: ResolverPolicy,
    transport: T,
}

impl<T: DohTransport> ProbeResolver<T> {
    #[must_use]
    pub fn new(policy: ResolverPolicy, transport: T) -> Self {
        Self { policy, transport }
    }

    /// Optional once-per-batch health filter. A wholly failed filter retains
    /// configured fallback order, matching configured_working_probe_resolvers.
    pub fn filter_working(
        &mut self,
        budget: Duration,
        cancelled: &dyn Fn() -> bool,
    ) -> Result<(), ResolveError> {
        let deadline = Instant::now() + budget.min(HOST_BUDGET);
        let mut working = Vec::new();
        for endpoint in &self.policy.endpoints {
            remaining(deadline, cancelled)?;
            let q = question("example.com", 1)?;
            match self
                .transport
                .query(endpoint, &q, deadline, cancelled)
                .and_then(|raw| q.addresses(&raw))
            {
                Ok(addresses) if !addresses.is_empty() => working.push(endpoint.clone()),
                Err(ResolveError::Cancelled) => return Err(ResolveError::Cancelled),
                _ => {}
            }
        }
        remaining(deadline, cancelled)?;
        if !working.is_empty() {
            self.policy.endpoints = working;
        }
        Ok(())
    }

    /// First resolver with any public A/AAAA answers wins; no ambient libc
    /// fallback. Empty output is unresolved, never a measured latency failure.
    pub fn resolve(
        &mut self,
        host: &str,
        budget: Duration,
        cancelled: &dyn Fn() -> bool,
    ) -> Result<Vec<IpAddr>, ResolveError> {
        self.resolve_with_clock(host, budget, cancelled, &Instant::now)
    }

    fn resolve_with_clock(
        &mut self,
        host: &str,
        budget: Duration,
        cancelled: &dyn Fn() -> bool,
        now: &dyn Fn() -> Instant,
    ) -> Result<Vec<IpAddr>, ResolveError> {
        let deadline = now() + budget.min(HOST_BUDGET);
        remaining_with_clock(deadline, cancelled, now)?;
        if let Ok(ip) = host.parse::<IpAddr>() {
            return Ok(if public_address(ip) {
                vec![ip]
            } else {
                Vec::new()
            });
        }
        canonical_name(host)?;
        for endpoint in &self.policy.endpoints {
            let mut result = Vec::new();
            for kind in [1, 28] {
                match remaining_with_clock(deadline, cancelled, now) {
                    Err(ResolveError::Timeout) if !result.is_empty() => return Ok(result),
                    Err(error) => return Err(error),
                    Ok(_) => {}
                }
                let q = question(host, kind)?;
                let reply = self.transport.query(endpoint, &q, deadline, cancelled);
                if matches!(reply, Err(ResolveError::Cancelled)) {
                    return Err(ResolveError::Cancelled);
                }
                // Preserve A pins obtained within the budget when a later AAAA
                // attempt exhausts it. Never accept late new data, continue to
                // another endpoint or override cancellation with partial success.
                match remaining_with_clock(deadline, cancelled, now) {
                    Err(ResolveError::Timeout) if !result.is_empty() => return Ok(result),
                    Err(error) => return Err(error),
                    Ok(_) => {}
                }
                match reply.and_then(|raw| q.addresses(&raw)) {
                    Ok(addresses) => {
                        for ip in addresses {
                            if result.len() < MAX_ADDRESSES && !result.contains(&ip) {
                                result.push(ip);
                            }
                        }
                    }
                    Err(ResolveError::Cancelled) => return Err(ResolveError::Cancelled),
                    _ => {}
                }
            }
            if cancelled() {
                return Err(ResolveError::Cancelled);
            }
            if !result.is_empty() {
                return Ok(result);
            }
        }
        Ok(Vec::new())
    }
}

/// No Debug: cached endpoint authorities remain private. This transport has no
/// thread, process, listener, proxy, redirect or system-resolver capability.
pub struct HttpsDohTransport {
    bootstrap: Vec<IpAddr>,
    pins: BTreeMap<String, Vec<IpAddr>>,
}

impl HttpsDohTransport {
    #[must_use]
    pub fn for_policy(policy: &ResolverPolicy) -> Self {
        Self {
            bootstrap: policy.bootstrap.clone(),
            pins: BTreeMap::new(),
        }
    }

    fn pins(
        &mut self,
        host: &str,
        deadline: Instant,
        cancelled: &dyn Fn() -> bool,
    ) -> Result<Vec<IpAddr>, ResolveError> {
        if let Ok(ip) = host.trim_matches(['[', ']']).parse::<IpAddr>() {
            return if public_address(ip) {
                Ok(vec![ip])
            } else {
                Err(ResolveError::Policy)
            };
        }
        if let Some(ips) = self.pins.get(host) {
            return Ok(ips.clone());
        }
        for bootstrap in &self.bootstrap {
            let mut ips = Vec::new();
            for kind in [1, 28] {
                remaining(deadline, cancelled)?;
                let q = question(host, kind)?;
                match bootstrap_query(*bootstrap, &q, deadline, cancelled)
                    .and_then(|raw| q.addresses(&raw))
                {
                    Ok(values) => {
                        for ip in values {
                            if ips.len() < MAX_ADDRESSES && !ips.contains(&ip) {
                                ips.push(ip);
                            }
                        }
                    }
                    Err(ResolveError::Cancelled) => return Err(ResolveError::Cancelled),
                    _ => {}
                }
            }
            if !ips.is_empty() {
                if self.pins.len() >= MAX_RESOLVERS {
                    return Err(ResolveError::Policy);
                }
                self.pins.insert(host.to_owned(), ips.clone());
                return Ok(ips);
            }
        }
        Err(ResolveError::Unavailable)
    }
}

struct PinnedResolver {
    host: String,
    addresses: Vec<IpAddr>,
}
impl fmt::Debug for PinnedResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PinnedProbeResolver")
    }
}
impl Resolver for PinnedResolver {
    fn resolve(
        &self,
        uri: &ureq::http::Uri,
        _config: &ureq::config::Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        if uri.host() != Some(self.host.as_str())
            || uri.scheme_str() != Some("https")
            || uri.port_u16().unwrap_or(443) != 443
        {
            return Err(ureq::Error::HostNotFound);
        }
        let mut out = self.empty();
        for ip in &self.addresses {
            out.push(SocketAddr::new(*ip, 443));
        }
        if out.is_empty() {
            return Err(ureq::Error::HostNotFound);
        }
        Ok(out)
    }
}

impl DohTransport for HttpsDohTransport {
    fn query(
        &mut self,
        endpoint: &DohEndpoint,
        question: &DnsQuestion,
        deadline: Instant,
        cancelled: &dyn Fn() -> bool,
    ) -> Result<Vec<u8>, ResolveError> {
        remaining(deadline, cancelled)?;
        let host = endpoint.0.host_str().ok_or(ResolveError::Policy)?;
        let addresses = self.pins(host, deadline, cancelled)?;
        let timeout = remaining(deadline, cancelled)?.min(EXCHANGE_BUDGET);
        let config = Agent::config_builder()
            .timeout_global(Some(timeout))
            .max_redirects(0)
            .http_status_as_error(false)
            .max_response_header_size(8192)
            .user_agent("OmaVLESS-Probe/1")
            .accept("application/dns-message")
            .accept_encoding("identity")
            .proxy(None)
            .tls_config(
                TlsConfig::builder()
                    .root_certs(RootCerts::PlatformVerifier)
                    .build(),
            )
            .build();
        let agent = Agent::with_parts(
            config,
            DefaultConnector::default(),
            PinnedResolver {
                host: host.to_owned(),
                addresses,
            },
        );
        let mut response = agent
            .post(endpoint.0.as_str())
            .header("Content-Type", "application/dns-message")
            .send(question.wire())
            .map_err(|_| ResolveError::Unavailable)?;
        if response.status().as_u16() != 200
            || response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .is_none_or(|s| {
                    s.split(';').next().unwrap_or("").trim() != "application/dns-message"
                })
        {
            return Err(ResolveError::InvalidResponse);
        }
        let bytes = response
            .body_mut()
            .with_config()
            .limit((MAX_DNS_BYTES + 1) as u64)
            .read_to_vec()
            .map_err(|_| ResolveError::Unavailable)?;
        remaining(deadline, cancelled)?;
        if bytes.len() > MAX_DNS_BYTES {
            return Err(ResolveError::InvalidResponse);
        }
        Ok(bytes)
    }
}

fn bootstrap_query(
    ip: IpAddr,
    question: &DnsQuestion,
    deadline: Instant,
    cancelled: &dyn Fn() -> bool,
) -> Result<Vec<u8>, ResolveError> {
    let budget = remaining(deadline, cancelled)?.min(EXCHANGE_BUDGET);
    let deadline = Instant::now() + budget;
    let bind = match ip {
        IpAddr::V4(_) => "0.0.0.0:0",
        IpAddr::V6(_) => "[::]:0",
    };
    let socket = UdpSocket::bind(bind).map_err(|_| ResolveError::Unavailable)?;
    socket
        .connect(SocketAddr::new(ip, 53))
        .map_err(|_| ResolveError::Unavailable)?;
    socket
        .set_write_timeout(Some(budget))
        .map_err(|_| ResolveError::Unavailable)?;
    socket
        .send(question.wire())
        .map_err(|_| ResolveError::Unavailable)?;
    let mut bytes = vec![0; MAX_DNS_BYTES + 1];
    loop {
        socket
            .set_read_timeout(Some(
                remaining(deadline, cancelled)?.min(Duration::from_millis(100)),
            ))
            .map_err(|_| ResolveError::Unavailable)?;
        match socket.recv(&mut bytes) {
            Ok(length) if length <= MAX_DNS_BYTES => {
                bytes.truncate(length);
                return Ok(bytes);
            }
            Ok(_) => return Err(ResolveError::InvalidResponse),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::Interrupted
                ) => {}
            Err(_) => return Err(ResolveError::Unavailable),
        }
    }
}

#[cfg(test)]
#[path = "native_probe_resolver_tests.rs"]
mod tests;
