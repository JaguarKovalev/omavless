// SPDX-License-Identifier: MIT
//! Private detached DNS + auxiliary-core work. No store writes or UI payloads.

use crate::auxiliary_core::AuxiliaryLease;
use crate::native_coordinator::NativeSubscriptionProbe;
use crate::native_probe_resolver::{
    HOST_BUDGET, HttpsDohTransport, ProbeResolver, ResolveError, ResolverPolicy,
};
use crate::probe_executor::{MAX_JOB_TIME, ProbeExecutionError, execute_plan};
use crate::remote_fetch::RemoteFetchPool;
use omavless_control_protocol::StableErrorCode;
use omavless_mihomo::probe_plan::{PinnedProfile, ProbePlan, ProbeResult};
use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, Instant};

pub(crate) fn execute(
    job: &NativeSubscriptionProbe,
    lease: &AuxiliaryLease,
    core: &Path,
    scratch: &Path,
    pool: &RemoteFetchPool,
    valid: &dyn Fn() -> bool,
) -> Result<Vec<ProbeResult>, StableErrorCode> {
    let deadline = Instant::now() + MAX_JOB_TIME;
    let cancelled = || !valid();
    let _permit = loop {
        check(deadline, valid)?;
        if let Some(permit) = pool.try_acquire() {
            break permit;
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let policy =
        ResolverPolicy::from_trusted_configs(job.private_active_config(), job.private_template())
            .map_err(|_| StableErrorCode::CapabilityUnavailable)?;
    let transport = HttpsDohTransport::for_policy(&policy);
    let mut resolver = ProbeResolver::new(policy, transport);
    // Filter only when names require DNS. Failure retains the configured order;
    // it is not proof that any VPN server is unreachable.
    if job
        .profiles()
        .iter()
        .any(|(_, p)| p.private_endpoint().parse::<std::net::IpAddr>().is_err())
    {
        check(deadline, valid)?;
        let budget = Duration::from_secs(5).min(deadline.saturating_duration_since(Instant::now()));
        let _ = resolver.filter_working(budget, &cancelled);
        check(deadline, valid)?;
    }
    let mut cache: BTreeMap<String, Vec<std::net::IpAddr>> = BTreeMap::new();
    let mut addresses = Vec::with_capacity(job.profiles().len());
    for (_, profile) in job.profiles() {
        check(deadline, valid)?;
        let host = profile.private_endpoint();
        let pins = if let Some(pins) = cache.get(host) {
            pins.clone()
        } else {
            let budget = HOST_BUDGET.min(deadline.saturating_duration_since(Instant::now()));
            let pins = match resolver.resolve(host, budget, &cancelled) {
                Ok(pins) => pins,
                Err(ResolveError::Cancelled) => return Err(StableErrorCode::Conflict),
                // A failed bounded DNS attempt is unresolved, never measured
                // provider failure. No libc or ambient DNS escape hatch.
                Err(_) => Vec::new(),
            };
            cache.insert(host.to_owned(), pins.clone());
            pins
        };
        addresses.push(pins);
    }
    check(deadline, valid)?;
    let pinned: Vec<_> = job
        .profiles()
        .iter()
        .zip(&addresses)
        .map(|((_, profile), addresses)| PinnedProfile { profile, addresses })
        .collect();
    let plan = ProbePlan::new(&pinned).map_err(|_| StableErrorCode::CoreRejected)?;
    execute_plan(&plan, core, scratch, lease, deadline, cancelled, |_, _| {})
        .map_err(execution_error)
}

fn check(deadline: Instant, valid: &dyn Fn() -> bool) -> Result<(), StableErrorCode> {
    if !valid() {
        return Err(StableErrorCode::Conflict);
    }
    if Instant::now() >= deadline {
        return Err(StableErrorCode::CoreRejected);
    }
    Ok(())
}

fn execution_error(error: ProbeExecutionError) -> StableErrorCode {
    match error {
        ProbeExecutionError::CleanupRequired => StableErrorCode::ManualRecoveryRequired,
        ProbeExecutionError::Cancelled => StableErrorCode::Conflict,
        ProbeExecutionError::Unavailable => StableErrorCode::CapabilityUnavailable,
        _ => StableErrorCode::CoreRejected,
    }
}
