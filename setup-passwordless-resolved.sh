#!/bin/bash
# SPDX-License-Identifier: MIT
# One-time host setup for passwordless per-link DNS changes used by Mihomo TUN.
set -euo pipefail

user_name="${SUDO_USER:-${USER}}"
rule_path="/etc/polkit-1/rules.d/49-omavless-resolved-${user_name}.rules"

cat <<EOF | sudo tee "$rule_path" >/dev/null
polkit.addRule(function(action, subject) {
    if (subject.user !== "${user_name}")
        return polkit.Result.NOT_HANDLED;

    if (action.id === "org.freedesktop.resolve1.set-domains" ||
        action.id === "org.freedesktop.resolve1.set-default-route" ||
        action.id === "org.freedesktop.resolve1.set-dns-servers" ||
        action.id === "org.freedesktop.resolve1.revert") {
        return polkit.Result.YES;
    }

    return polkit.Result.NOT_HANDLED;
});
EOF

sudo chmod 0644 "$rule_path"
echo "Installed $rule_path"
echo "Mihomo may now set and revert DNS/domain/default-route for TUN links without repeated authentication prompts."
