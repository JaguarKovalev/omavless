// SPDX-License-Identifier: MIT
//! Bounded shareable report, distinct from live rule/provider diagnostics.
use crate::desired::DesiredState;
use crate::lifecycle::{ActualState, NativeLocalObservation};
use crate::mutation_protocol::MutationProtocolError;
use omavless_control_protocol::validate_request;
use serde_json::{Value, json};

pub(crate) fn validate(request: &Value) -> Result<(), MutationProtocolError> {
    validate_request(request).map_err(|_| MutationProtocolError::InvalidRequest)?;
    if request["method"] != "diagnostics.export" {
        return Err(MutationProtocolError::UnknownMethod);
    }
    if !request["params"].as_object().is_some_and(|p| p.is_empty()) {
        return Err(MutationProtocolError::InvalidArgument);
    }
    Ok(())
}

pub(crate) fn report(
    configuration: Value,
    desired: &DesiredState,
    actual: ActualState,
    pending: bool,
    observation: Option<NativeLocalObservation>,
) -> Value {
    // Saturated/inconsistent observations are unavailable, never rounded into
    // healthy counts. The same bounds are enforced by the presentation client.
    let observation = observation.filter(|o| {
        o.visible_mihomo_count <= 64
            && o.visible_tun_count <= 8
            && o.owned_auxiliary_mihomo_count <= 1
            && o.owned_auxiliary_mihomo_count <= o.visible_mihomo_count
            && (!o.desired_profile_matches_owned || (o.owned_core_running && desired.connected))
            && (!o.owned_controller_config_verified
                || (o.owned_core_running && o.desired_profile_matches_owned))
    });
    let actual = match actual {
        ActualState::Disconnected => "disconnected",
        ActualState::Starting => "starting",
        ActualState::Connected => "connected",
        ActualState::Reconnecting => "reconnecting",
        ActualState::Stopping => "stopping",
        ActualState::Failed => "failed",
        ActualState::ManualRecoveryRequired => "manual_recovery_required",
    };
    json!({
        "schemaVersion": 2,
        "scope": "native_support",
        "runtime": {
            "implementation": "rust",
            "version": env!("CARGO_PKG_VERSION"),
            "lastKnownState": actual,
            "routingTransactionPending": pending,
        },
        "configuration": configuration,
        "localObservation": {
            "availability": if observation.is_some() { "observed" } else { "unavailable" },
            "desired": { "connected": desired.connected, "mode": desired.mode.as_str() },
            "facts": observation.map(|o| json!({
                "ownedCoreRunning": o.owned_core_running,
                "visibleMihomoCount": o.visible_mihomo_count,
                "ownedAuxiliaryMihomoCount": o.owned_auxiliary_mihomo_count,
                "visibleTunCount": o.visible_tun_count,
                "ownedControllerConfigVerified": o.owned_controller_config_verified,
                "desiredProfileMatchesOwned": o.desired_profile_matches_owned,
            })),
            "verification": { "serviceOwnership": false, "tunOwnership": false,
                "routes": false, "dns": false, "internet": false },
        },
        "coverage": {
            "privateStoreValidated": true,
            "liveHostObservation": observation.is_some(),
            "controllerQuery": observation.is_some_and(|o| o.owned_controller_config_verified),
            "loginActivationVerified": false,
            "coreSetupVerified": false,
            "serviceEnablementVerified": false,
            "loadedPolicyCounts": false,
            "fileReadiness": false,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use omavless_control_protocol::make_request;
    #[test]
    fn support_requests_reject_all_client_data_without_echo() {
        assert!(
            validate(&make_request("support", "diagnostics.export", json!({})).unwrap()).is_ok()
        );
        for params in [
            json!({"path":"private-token"}),
            json!({"raw":true}),
            json!({"operationId":"private-token"}),
            json!({"expectedRevision":0}),
        ] {
            let error = validate(&make_request("support", "diagnostics.export", params).unwrap())
                .unwrap_err();
            assert!(!format!("{error} {error:?}").contains("private-token"));
        }
        assert!(
            validate(&make_request("support", "diagnostics.summary", json!({})).unwrap()).is_err()
        );
    }

    #[test]
    fn support_state_is_explicitly_last_known_and_recovery_is_not_disconnected() {
        let store = omavless_domain::private_store::parse_private_store(
            r#"{"version":3,"profiles":[],"subscriptions":[]}"#,
        )
        .unwrap();
        let value = report(
            store.support_projection(),
            &DesiredState::default(),
            ActualState::ManualRecoveryRequired,
            true,
            None,
        );
        assert_eq!(
            value["runtime"]["lastKnownState"],
            "manual_recovery_required"
        );
        assert_eq!(value["runtime"]["routingTransactionPending"], true);
        assert_eq!(value["coverage"]["liveHostObservation"], false);
        assert!(value.to_string().len() < 4096);
    }

    #[test]
    fn support_observation_releases_only_typed_facts_not_profile_or_live_health() {
        let desired = DesiredState {
            connected: true,
            profile_id: "private-token".into(),
            ..DesiredState::default()
        };
        let observation = NativeLocalObservation {
            owned_core_running: true,
            visible_mihomo_count: 2,
            owned_auxiliary_mihomo_count: 1,
            visible_tun_count: 1,
            owned_controller_config_verified: true,
            desired_profile_matches_owned: true,
        };
        let store = omavless_domain::private_store::parse_private_store(
            r#"{"version":3,"profiles":[],"subscriptions":[]}"#,
        )
        .unwrap();
        let value = report(
            store.support_projection(),
            &desired,
            ActualState::Failed,
            false,
            Some(observation),
        );
        assert_eq!(value["localObservation"]["availability"], "observed");
        assert_eq!(value["runtime"]["lastKnownState"], "failed");
        assert_eq!(value["coverage"]["controllerQuery"], true);
        assert!(
            value["localObservation"]["verification"]
                .as_object()
                .unwrap()
                .values()
                .all(|v| v == false)
        );
        for forbidden in [
            "private-token",
            "profileId",
            "instanceId",
            "generation",
            "192.0.2",
            "://",
        ] {
            assert!(!value.to_string().contains(forbidden));
        }
        assert!(value.to_string().len() < 4096);
    }

    #[test]
    fn inconsistent_and_saturated_support_facts_are_unavailable_not_rounded() {
        let mut cases = Vec::new();
        let empty = NativeLocalObservation {
            owned_core_running: false,
            visible_mihomo_count: 0,
            owned_auxiliary_mihomo_count: 0,
            visible_tun_count: 0,
            owned_controller_config_verified: false,
            desired_profile_matches_owned: false,
        };
        cases.push(NativeLocalObservation {
            visible_mihomo_count: 65,
            ..empty
        });
        cases.push(NativeLocalObservation {
            visible_tun_count: 9,
            ..empty
        });
        cases.push(NativeLocalObservation {
            owned_auxiliary_mihomo_count: 1,
            ..empty
        });
        cases.push(NativeLocalObservation {
            owned_controller_config_verified: true,
            ..empty
        });
        cases.push(NativeLocalObservation {
            desired_profile_matches_owned: true,
            ..empty
        });
        for observation in cases {
            let value = report(
                json!({}),
                &DesiredState::default(),
                ActualState::Disconnected,
                false,
                Some(observation),
            );
            assert_eq!(value["localObservation"]["availability"], "unavailable");
            assert!(value["localObservation"]["facts"].is_null());
            assert_eq!(value["coverage"]["liveHostObservation"], false);
        }
    }
}
