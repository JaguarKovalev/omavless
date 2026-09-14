#!/bin/bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 OmaVLESS contributors
set -euo pipefail

plugin_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
plugins_dir="$HOME/.config/omarchy/plugins"
target="$plugins_dir/kdk.omavless"
fresh_install=false
[[ -e "$target" ]] || fresh_install=true

omarchy plugin validate "$plugin_dir"
mkdir -p "$plugins_dir"

stage="$(mktemp -d "$plugins_dir/.kdk.omavless.install.XXXXXX")"
backup=""

cleanup() {
  [[ ! -d "$stage" ]] || rm -rf -- "$stage"
  if [[ -n "$backup" && -d "$backup" && ! -e "$target" ]]; then
    mv -- "$backup" "$target"
  fi
}
trap cleanup EXIT

cp -a \
  "$plugin_dir/Panel.qml" \
  "$plugin_dir/Service.qml" \
  "$plugin_dir/PlainText.qml" \
  "$plugin_dir/Sparkline.qml" \
  "$plugin_dir/NamePrompt.qml" \
  "$plugin_dir/ImportPreviewPrompt.qml" \
  "$plugin_dir/SubscriptionPrompt.qml" \
  "$plugin_dir/RoutingPresetPrompt.qml" \
  "$plugin_dir/OnboardingWizard.qml" \
  "$plugin_dir/StartupPrompt.qml" \
  "$plugin_dir/RoutingToolsPrompt.qml" \
  "$plugin_dir/RenameWindow.qml" \
  "$plugin_dir/QrWindow.qml" \
  "$plugin_dir/backend.sh" \
  "$plugin_dir/backend.py" \
  "$plugin_dir/control.py" \
  "$plugin_dir/uninstall.sh" \
  "$plugin_dir/manifest.json" \
  "$plugin_dir/preview.png" \
  "$plugin_dir/LICENSE" \
  "$plugin_dir/THIRD_PARTY_NOTICES.md" \
  "$plugin_dir/README.md" \
  "$plugin_dir/CHANGELOG.md" \
  "$stage/"
mkdir -p "$stage/templates"
cp -a "$plugin_dir/templates/." "$stage/templates/"
chmod 755 "$stage/backend.sh" "$stage/backend.py" "$stage/control.py" "$stage/uninstall.sh"

if [[ -e "$target" ]]; then
  backup="${stage}.backup"
  mv -- "$target" "$backup"
fi
mv -- "$stage" "$target"

if [[ "$fresh_install" == true ]]; then
  discovered=false
  for _ in {1..30}; do
    if omarchy plugin list --json 2>/dev/null | jq -e \
      'any(.[]; .id == "kdk.omavless")' >/dev/null; then
      discovered=true
      break
    fi
    sleep 0.1
  done

  if [[ "$discovered" == false ]]; then
    omarchy-shell shell rescanPlugins
    for _ in {1..30}; do
      if omarchy plugin list --json 2>/dev/null | jq -e \
        'any(.[]; .id == "kdk.omavless")' >/dev/null; then
        discovered=true
        break
      fi
      sleep 0.1
    done
  fi

  [[ "$discovered" == true ]] || {
    echo "OmaVLESS was copied but the running shell did not discover it." >&2
    exit 1
  }
  omarchy plugin enable kdk.omavless right
fi

if [[ -n "$backup" && -d "$backup" ]]; then
  rm -rf -- "$backup"
  backup=""
fi
trap - EXIT

echo "OmaVLESS is installed or updated; no tunnel was started."
