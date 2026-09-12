#!/bin/bash
# TEST TOOL ONLY: run by the reviewed disposable-account host wrapper.
# No real fixture, VPN connection, network fetch or system configuration write.
set -euo pipefail
umask 077
[[ $# == 1 && $1 == --run && $EUID -ne 0 &&
   ${HOME:-} == /home/omavless-r6-domain &&
   $(id -un) == omavless-r6-domain &&
   ${XDG_RUNTIME_DIR:-} == /run/user/"$EUID" &&
   ! ${OMAVLESS_HOME+x} ]] || { printf '%s\n' 'DOMAIN GATE REFUSED: disposable account required'; exit 2; }

stage=python_absence
scratch=$(mktemp -d "$HOME/domain-evidence.XXXXXX")
exec 2>"$scratch/errors"
trap 'code=$?; if ((code)); then printf "DOMAIN GATE FAIL: %s\n" "$stage"; fi' EXIT
for interpreter in /usr/bin/python /usr/bin/python3 /usr/bin/python3.*; do
  [[ -e $interpreter || -L $interpreter ]] || continue
  if "$interpreter" -c 'raise SystemExit(0)' >/dev/null 2>&1; then exit 1; fi
done
! command -v python >/dev/null 2>&1
! command -v python3 >/dev/null 2>&1

cli() {
  timeout --kill-after=1s 15s /usr/bin/omavless "$@" >"$scratch/response.json" 2>"$scratch/response.err"
  [[ $(stat -c %s "$scratch/response.json") -le 262144 ]]
  jq -e '.ok == true' "$scratch/response.json" >/dev/null
}
snapshot() { cli plugin snapshot; }
empty_host() {
  cli runtime observation
  jq -e '.result.availability=="observed" and .result.lastKnownActual=="disconnected"
    and .result.desired.connected==false and .result.manualRecoveryRequired==false
    and .result.facts.ownedCoreRunning==false and .result.facts.visibleMihomoCount==0
    and .result.facts.visibleTunCount==0 and .result.facts.ownedAuxiliaryMihomoCount==0' "$scratch/response.json" >/dev/null
}
operation=0
action() {
  local name=$1
  shift
  case $name in
    profile-import|profile-rename|profile-favorite|profile-replace|profile-delete|custom-rule-add|custom-rule-delete|onboarding-complete|mode) ;;
    *) return 1 ;;
  esac
  snapshot
  local instance revision
  instance=$(jq -er '.result.instanceId' "$scratch/response.json")
  revision=$(jq -er '.revision' "$scratch/response.json")
  operation=$((operation+1))
  cli plugin "$name" "$instance" "$revision" "r6-domain-$operation" "$@"
}
pass() { printf 'DOMAIN PASS: %s\n' "$stage"; }
stage=empty_native_baseline
[[ $(/usr/bin/omavless plugin target) == rust ]]
empty_host
snapshot
jq -e '(.result.profiles|length)==0 and (.result.subscriptions|length)==0
  and .result.startup.enabled==false' "$scratch/response.json" >/dev/null
pass

# Public reserved-address synthetic input, never a live protocol fixture.
profile='vless://00000000-0000-4000-8000-000000000001@192.0.2.1:443?security=tls'
stage=profile_preview
cli import preview <<<"$profile"
jq -e '.result.kind=="profile"' "$scratch/response.json" >/dev/null
pass
stage=subscription_preview_without_fetch
cli import preview <<<'https://subscription.example.invalid/list'
jq -e '.result.kind=="subscription" and .result.duplicate==false' "$scratch/response.json" >/dev/null
pass
stage=profile_import
action profile-import <<<"Synthetic import"$'\n'"$profile"
snapshot
jq -e '(.result.profiles|length)==1 and .result.profiles[0].name=="Synthetic import"' "$scratch/response.json" >/dev/null
profile_id=$(jq -er '.result.profiles[0].id' "$scratch/response.json")
pass
stage=profile_rename_favorite
action profile-rename <<<"$profile_id"$'\nSynthetic renamed'
action profile-favorite <<<"$profile_id"$'\non'
snapshot
jq -e '.result.profiles[0].name=="Synthetic renamed" and .result.profiles[0].favorite==true' "$scratch/response.json" >/dev/null
pass
stage=profile_editor_export
cli profile edit-input "$profile_id"
cli profile export "$profile_id" file
jq -e '.result.format=="uri" and (.result.content|startswith("vless://"))' "$scratch/response.json" >/dev/null
pass
stage=profile_replace
action profile-replace <<<"$profile_id"$'\nSynthetic replacement\n'"${profile/192.0.2.1/192.0.2.2}"
snapshot
jq -e '(.result.profiles|length)==1 and .result.profiles[0].name=="Synthetic replacement"' "$scratch/response.json" >/dev/null
pass
stage=custom_rule_add_delete
action custom-rule-add <<<$'domain\ndirect\nexample.invalid'
cli routing rules
jq -e '(.result.rules|length)==1' "$scratch/response.json" >/dev/null
rule_id=$(jq -er '.result.rules[0].id' "$scratch/response.json")
action custom-rule-delete <<<"$rule_id"
cli routing rules
jq -e '(.result.rules|length)==0' "$scratch/response.json" >/dev/null
pass
stage=disconnected_modes
for mode in global direct rule; do
  action mode "$mode"
  empty_host
  jq -e --arg mode "$mode" '.result.desired.mode==$mode' "$scratch/response.json" >/dev/null
done
pass
stage=onboarding_support
action onboarding-complete
cli diagnostics export
jq -e '(.result.schemaVersion==2 or .result.schemaVersion==3) and .result.scope=="native_support"
  and .result.configuration.onboardingComplete==true
  and .result.coverage.liveHostObservation==true' "$scratch/response.json" >/dev/null
pass
stage=profile_delete
action profile-delete <<<"$profile_id"
snapshot
jq -e '(.result.profiles|length)==0 and (.result.subscriptions|length)==0' "$scratch/response.json" >/dev/null
pass
stage=final_empty_native_host
empty_host
[[ $(stat -c %a "$HOME/.config/omavless/profiles.json") == 600 ]]
pass
printf '%s\n' 'INSTALLED DOMAIN MATRIX PASS: no Python, synthetic inputs, no VPN/GUI/provider evidence'
