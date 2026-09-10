// SPDX-License-Identifier: MIT
// Presentation of validated native metadata. No mutation or lifecycle ownership.
function project(snapshot, observation, failed) {
  var result = {state:"unavailable", connected:false, activeId:"", mode:"rule",
    profiles:[], subscriptions:[], lastProfileId:""}
  if (!snapshot) return result
  result.profiles = snapshot.profiles.slice(0, 256)
  result.subscriptions = snapshot.subscriptions.slice(0, 64)
  result.lastProfileId = snapshot.lastProfileId
  result.mode = snapshot.desired.mode
  if (failed || !observation || observation.availability !== "observed"
      || observation.instanceId !== snapshot.instanceId
      || observation.revision !== snapshot.revision
      || observation.lastKnownActual !== snapshot.lastKnownActual
      || observation.desired.generation !== snapshot.desired.generation
      || observation.desired.connected !== snapshot.desired.connected
      || observation.desired.mode !== snapshot.desired.mode) return result
  var facts = observation.facts
  if (!facts) return result
  // The visible inventory stays truthful: only the explicitly attributed
  // disposable no-TUN child is separate from the one requested tunnel core.
  var auxiliary = facts.ownedAuxiliaryMihomoCount === undefined ? 0 : facts.ownedAuxiliaryMihomoCount
  if ([0, 1].indexOf(auxiliary) < 0 || auxiliary > facts.visibleMihomoCount) return result
  var tunnelCores = facts.visibleMihomoCount - auxiliary
  result.state = snapshot.lastKnownActual
  if (result.state === "connected") {
    result.connected = snapshot.desired.connected && facts.ownedCoreRunning
      && facts.desiredProfileMatchesOwned && facts.ownedControllerConfigVerified
      && tunnelCores === 1 && facts.visibleTunCount === 1
    if (result.connected) result.activeId = snapshot.desired.profileId
    else result.state = "unavailable"
  } else if (result.state === "disconnected"
      && (snapshot.desired.connected || facts.ownedCoreRunning
          || tunnelCores !== 0 || facts.visibleTunCount !== 0)) {
    result.state = "unavailable"
  }
  return result
}

function filtered(profiles, query) {
  var needle = typeof query === "string" ? query.slice(0,128).toLowerCase() : ""
  return (profiles || []).slice(0,256).filter(function(p) {
    return p.name.toLowerCase().indexOf(needle) !== -1
  }).sort(function(a,b) {
    return Number(b.favorite) - Number(a.favorite)
  })
}
