#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Stable control-plane shim for the maintained OmaVLESS 0.7 fork.

The large upstream 0.7 backend remains the source of truth for profiles, routing
rules and the QML-facing CLI.  This shim owns the Linux-sensitive lifecycle:
service generation, TUN compatibility/readiness and Full VPN selector readiness.
"""

from __future__ import annotations

import contextlib
import time
from pathlib import Path

import backend


FORK_VERSION = "0.7.5"
SELECTOR_READY_TIMEOUT_SECONDS = 12.0
SELECTOR_POLL_SECONDS = 0.10
TUN_READY_TIMEOUT_SECONDS = 10.0

# Keep diagnostics and User-Agent aligned with the maintained fork version.
backend.PLUGIN_VERSION = FORK_VERSION
backend.USER_AGENT = f"OmaVLESS/{FORK_VERSION}"


# Keep references before monkey-patching the backend module.
_original_private_runtime_config = backend.private_runtime_config
_original_connect_profile = backend.connect_profile
_original_startup_connect = backend.startup_connect


def _strip_tun_route_excludes(text: str) -> str:
    """Drop route-exclude-address from the generated TUN config on Linux.

    Recent sing-tun/Mihomo nftables builds can reject the interval-set creation
    used by route-exclude-address with EEXIST ("netlink receive: file exists").
    Falling back to DISABLE_NFTABLES makes Mihomo spawn /usr/bin/iptables; that
    child does not inherit Mihomo's file CAP_NET_ADMIN and therefore fails on a
    normal user service with "Permission denied (you must be root)".

    OmaVLESS already has routing rules for private destinations.  For this
    maintained rootless fork, prefer the native in-process nftables auto-redirect
    path and omit this optional TUN optimisation until the upstream interval-set
    path is reliable on current Arch/Omarchy kernels.
    """
    lines = text.splitlines(keepends=True)
    output: list[str] = []
    in_tun = False
    skipping = False

    for line in lines:
        raw = line.rstrip("\r\n")
        stripped = raw.strip()
        indent = len(raw) - len(raw.lstrip(" "))

        if stripped and not stripped.startswith("#") and indent == 0:
            in_tun = stripped == "tun:" or stripped.startswith("tun: #")
            skipping = False

        if in_tun and indent == 2 and stripped.startswith("route-exclude-address:"):
            skipping = True
            continue

        if skipping:
            # List members and comments belong to route-exclude-address. Stop
            # once another key at the same (or higher) indentation is reached.
            if not stripped or stripped.startswith("#") or indent > 2:
                continue
            skipping = False

        output.append(line)

    return "".join(output)


def private_runtime_config(
    paths: backend.Paths, text: str, store: dict[str, object]
) -> str:
    return _original_private_runtime_config(paths, _strip_tun_route_excludes(text), store)


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

            # /version can become available before selector groups have finished
            # materialising. Do not PUT until the target is actually advertised.
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


def _tun_ready() -> bool:
    return (Path("/sys/class/net") / backend.TUN_DEVICE).exists()


def _wait_tun_ready(timeout: float = TUN_READY_TIMEOUT_SECONDS) -> bool:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if _tun_ready():
            return True
        time.sleep(0.10)
    return _tun_ready()


def connect_profile(paths: backend.Paths, profile_id: str) -> None:
    """Connect through the upstream transaction, but require a real TUN device."""
    _original_connect_profile(paths, profile_id)
    if _wait_tun_ready():
        return

    # Mihomo can keep its REST/mixed-port service alive even after TUN setup
    # failed. Do not report that state as a successful VPN connection.
    with contextlib.suppress(Exception):
        backend.stop_service(paths, profile_id)
    raise backend.BackendError(
        f"Mihomo started but TUN interface {backend.TUN_DEVICE} did not become ready"
    )


def startup_connect(paths: backend.Paths) -> None:
    _original_startup_connect(paths)
    if _wait_tun_ready():
        return
    with contextlib.suppress(Exception):
        backend.stop_service(paths)
    raise backend.BackendError(
        f"Mihomo started but TUN interface {backend.TUN_DEVICE} did not become ready"
    )


def _unit_condition_path(path: Path) -> str:
    """Return a systemd ConditionPathExists value without ExecStart-style quotes."""
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
# Keep Mihomo's native nftables auto-redirect path. The iptables fallback
# spawns an unprivileged child process and cannot use Mihomo's file capabilities.
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


# Patch only lifecycle-sensitive functions. Profile parsing, storage,
# subscriptions, routing templates and the QML-facing CLI stay upstream 0.7.
backend.private_runtime_config = private_runtime_config
backend.select_global_proxy = select_global_proxy
backend.ensure_unit = ensure_unit
backend.ensure_startup_unit = ensure_startup_unit
backend.connect_profile = connect_profile
backend.startup_connect = startup_connect


if __name__ == "__main__":
    raise SystemExit(backend.main())
