#!/bin/bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 OmaVLESS contributors
set -euo pipefail

purge=false
case "${1:-}" in
  "") ;;
  --purge) purge=true ;;
  *) echo "Usage: ./uninstall.sh [--purge]" >&2; exit 2 ;;
esac

# This is the legacy integration remover, not native package removal. Match
# CutoverPaths::current / backend.sh: even a stopped Rust runtime retains its
# ownership marker. Do not parse or repair that marker, and refuse every
# ownership artifact (including transition/invalid states) before any effects.
# A missing native executable must never make legacy purge safe by default.
legacy_ownership_proven_absent() {
  local uninstall_state_base remaining component current
  [[ ${HOME-} == /* && ${HOME-} != / && ${#HOME} -le 4096 ]] || return 1
  [[ $HOME != *[[:cntrl:]]* ]] || return 1
  if [[ ${XDG_STATE_HOME+x} == x ]]; then
    uninstall_state_base=$XDG_STATE_HOME
  else
    uninstall_state_base=$HOME/.local/state
  fi
  [[ $uninstall_state_base == /* && ${#uninstall_state_base} -le 4096 ]] || return 1
  [[ $uninstall_state_base != *[[:cntrl:]]* ]] || return 1
  # Check all lexical components before accepting an absent ancestor. In
  # particular, /missing/../existing must not short-circuit the guard.
  for remaining in "$HOME" "$uninstall_state_base"; do
    case "/${remaining#/}/" in */./*|*/../*) return 1 ;; esac
  done
  remaining=${uninstall_state_base#/}/omavless
  current=
  while [[ -n $remaining ]]; do
    component=${remaining%%/*}
    case "$remaining" in */*) remaining=${remaining#*/} ;; *) remaining= ;; esac
    [[ -n $component ]] || continue
    current=$current/$component
    [[ ! -L $current ]] || return 1
    if [[ ! -e $current ]]; then return 0; fi
    [[ -d $current && -r $current && -x $current ]] || return 1
  done
  for component in ownership.json frontend-bridge.target; do
    [[ ! -e $current/$component && ! -L $current/$component ]] || return 1
  done
}

if ! legacy_ownership_proven_absent; then
  printf '%s\n' 'Legacy uninstall refused: native ownership is present or unavailable. Nothing was removed. This script does not support native package removal or purge.' >&2
  exit 1
fi

unit="$HOME/.config/systemd/user/omavless.service"
startup_unit="$HOME/.config/systemd/user/omavless-autostart.service"
data="$HOME/.config/omavless"

# Removal must not leave an enabled full-route TUN service behind. Failure is
# ignored only when the service is already absent/inactive; a service that is
# still running stops the cleanup with its unit and data intact.
disable_failed=false
systemctl --user disable --now omavless-autostart.service >/dev/null 2>&1 || disable_failed=true
systemctl --user disable --now omavless.service >/dev/null 2>&1 || disable_failed=true
if systemctl --user is-active --quiet omavless.service; then
  echo "omavless.service is still running; nothing was removed." >&2
  exit 1
fi
if [[ "$disable_failed" == true ]] && {
  systemctl --user is-enabled --quiet omavless.service \
    || systemctl --user is-enabled --quiet omavless-autostart.service;
}; then
  echo "An OmaVLESS service is still enabled; nothing was removed." >&2
  exit 1
fi

rm -f -- "$unit" "$startup_unit"
systemctl --user daemon-reload

if [[ "$purge" == true ]]; then
  rm -rf -- "$data"
  echo "OmaVLESS runtime integration and saved profiles were removed."
else
  echo "OmaVLESS runtime integration was removed; saved profiles remain in $data."
fi
