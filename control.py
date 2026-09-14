#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Stable control-plane shim for the maintained OmaVLESS 0.7 fork.

This module deliberately keeps the large upstream backend intact and patches only
runtime-sensitive pieces: systemd unit generation and Full VPN selector readiness.
"""

from __future__ import annotations

import os
import time
from pathlib import Path

import backend


FORK_VERSION = "0.7.2"
SELECTOR_READY_TIMEOUT_SECONDS = 12.0
SELECTOR_POLL_SECONDS = 0.10

# Keep diagnostics and User-Agent aligned with the maintained fork version.
backend.PLUGIN_VERSION = FORK_VERSION
backend.USER_AGENT = f"OmaVLESS/{FORK_VERSION}"


def _wait_and_select(paths: backend.Paths, selector: str, target: str) -> None:
    """Wait for a selector to advertise a target, select it, then verify readback."""
    socket_path = backend.wait_private_controller(paths, timeout=10.0)
    endpoint = "/proxies/" + backend.urllib.parse.quote(selector, safe="")
    deadline = time.monotonic() + SELECTOR_READY_TIMEOUT_SECONDS

    while time.monotonic() < deadline:
        try:
            status_code, payload = backend.controller_json(socket_path, endpoint, 1.0)
            members = payload.get("all") if isinstance(payload, dict) else None
            current = payload.get("now") if isinstance(payload, dict) else None

            # Mihomo can expose /version before selector groups have finished
            # materialising.  Do not spam rejected PUT requests before the
            # requested member is actually advertised by the group.
            if status_code == 200 and isinstance(members, list) and target in members:
                if current == target:
                    return

                put_status, _ = backend.controller_request(
                    socket_path, "PUT", endpoint, 1.0, {"name": target}
                )
                if put_status == 204:
                    verify_status, verify = backend.controller_json(
                        socket_path, endpoint, 1.0
                    )
                    if (
                        verify_status == 200
                        and isinstance(verify, dict)
                        and verify.get("now") == target
                    ):
                        return
        except (backend.BackendError, OSError):
            pass

        time.sleep(SELECTOR_POLL_SECONDS)

    raise backend.BackendError(
        f"Mihomo Full VPN selector {selector} did not become ready"
    )


def select_global_proxy(paths: backend.Paths, profile_name: str) -> None:
    """Enforce the generated GLOBAL -> PROXY -> profile topology deterministically."""
    _wait_and_select(paths, "PROXY", profile_name)
    _wait_and_select(paths, "GLOBAL", "PROXY")


def _unit_condition_path(path: Path) -> str:
    """ConditionPathExists does not need ExecStart-style quoting for our fixed path."""
    value = str(path)
    if any(ch in value for ch in "\n\r\t"):
        raise backend.BackendError("Plugin path contains unsupported characters")
    return value


def ensure_unit(paths: backend.Paths, core: Path) -> None:
    launcher = backend.PLUGIN_DIR / "backend.sh"
    manifest = backend.PLUGIN_DIR / "manifest.json"
    unit = f"""[Unit]
Description=OmaVLESS tunnel (Mihomo core)
Wants=network-online.target
After=network-online.target
ConditionPathExists={_unit_condition_path(manifest)}

[Service]
ExecStart={backend.systemd_quote(str(launcher))} run-core {backend.systemd_quote(str(core))}
Restart=on-failure
RestartSec=2

[Install]
WantedBy=default.target
"""
    if paths.unit.is_symlink():
        raise backend.BackendError(f"Refusing symlinked systemd unit: {paths.unit}")
    current = (
        backend.read_text_file(paths.unit, 64 * 1024, "systemd unit")
        if paths.unit.exists()
        else ""
    )
    if current != unit:
        backend.atomic_write(paths.unit, unit, 0o644)
        backend.systemctl("daemon-reload")


def ensure_startup_unit(paths: backend.Paths) -> None:
    helper = backend.startup_unit(paths)
    launcher = backend.PLUGIN_DIR / "backend.sh"
    manifest = backend.PLUGIN_DIR / "manifest.json"
    unit = f"""[Unit]
Description=OmaVLESS login autoconnect
Wants=network-online.target
After=network-online.target
ConditionPathExists={_unit_condition_path(manifest)}

[Service]
Type=oneshot
ExecStart={backend.systemd_quote(str(launcher))} startup-connect

[Install]
WantedBy=default.target
"""
    if helper.is_symlink():
        raise backend.BackendError(f"Refusing symlinked systemd unit: {helper}")
    current = (
        backend.read_text_file(helper, 64 * 1024, "autostart systemd unit")
        if helper.exists()
        else ""
    )
    if current != unit:
        backend.atomic_write(helper, unit, 0o644)
        backend.systemctl("daemon-reload")


# Patch only the lifecycle-sensitive functions. All profile parsing, storage,
# subscriptions, routing templates and QML-facing CLI remain upstream 0.7 code.
backend.select_global_proxy = select_global_proxy
backend.ensure_unit = ensure_unit
backend.ensure_startup_unit = ensure_startup_unit


if __name__ == "__main__":
    raise SystemExit(backend.main())
