#!/usr/bin/env python3
from __future__ import annotations

import sys
import unittest
from pathlib import Path
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import control


class StripTunRouteExcludesTests(unittest.TestCase):
    def test_removes_only_route_exclude_block(self) -> None:
        source = """tun:\n  enable: true\n  auto-route: true\n  route-exclude-address:\n    - 10.0.0.0/8\n    - 192.168.0.0/16\n  strict-route: true\n\ndns:\n  enable: true\n"""
        result = control._strip_tun_route_excludes(source)
        self.assertNotIn("route-exclude-address", result)
        self.assertNotIn("10.0.0.0/8", result)
        self.assertNotIn("192.168.0.0/16", result)
        self.assertIn("  auto-route: true\n", result)
        self.assertIn("  strict-route: true\n", result)
        self.assertIn("dns:\n  enable: true\n", result)

    def test_leaves_config_without_excludes_unchanged(self) -> None:
        source = "tun:\n  enable: true\n  auto-route: true\n  strict-route: true\n"
        self.assertEqual(control._strip_tun_route_excludes(source), source)


class SelectorReadinessTests(unittest.TestCase):
    def test_waits_for_member_before_selector_put(self) -> None:
        payloads = [
            (200, {"all": [], "now": ""}),
            (200, {"all": ["profile-a"], "now": ""}),
            (200, {"all": ["profile-a"], "now": "profile-a"}),
        ]
        with (
            mock.patch.object(control.backend, "wait_private_controller", return_value=Path("/tmp/controller.sock")),
            mock.patch.object(control.backend, "controller_json", side_effect=payloads) as get_proxy,
            mock.patch.object(control.backend, "controller_request", return_value=(204, b"")) as put_proxy,
            mock.patch.object(control.time, "sleep"),
        ):
            control._wait_and_select(mock.sentinel.paths, "PROXY", "profile-a")

        self.assertEqual(get_proxy.call_count, 3)
        put_proxy.assert_called_once()
        self.assertEqual(put_proxy.call_args.args[1], "PUT")
        self.assertEqual(put_proxy.call_args.args[4], {"name": "profile-a"})


class TunReadinessTests(unittest.TestCase):
    def test_connect_rejects_mihomo_without_real_tun(self) -> None:
        with (
            mock.patch.object(control, "_original_connect_profile") as upstream_connect,
            mock.patch.object(control, "_wait_tun_ready", return_value=False),
            mock.patch.object(control.backend, "stop_service") as stop_service,
        ):
            with self.assertRaisesRegex(control.backend.BackendError, "TUN interface"):
                control.connect_profile(mock.sentinel.paths, "profile-id")

        upstream_connect.assert_called_once_with(mock.sentinel.paths, "profile-id")
        stop_service.assert_called_once_with(mock.sentinel.paths, "profile-id")

    def test_connect_accepts_real_tun(self) -> None:
        with (
            mock.patch.object(control, "_original_connect_profile") as upstream_connect,
            mock.patch.object(control, "_wait_tun_ready", return_value=True),
            mock.patch.object(control.backend, "stop_service") as stop_service,
        ):
            control.connect_profile(mock.sentinel.paths, "profile-id")

        upstream_connect.assert_called_once_with(mock.sentinel.paths, "profile-id")
        stop_service.assert_not_called()


class RuntimeConfigTests(unittest.TestCase):
    def test_runtime_config_strips_excludes_before_upstream_render(self) -> None:
        source = "tun:\n  enable: true\n  route-exclude-address:\n    - 10.0.0.0/8\n  strict-route: true\n"
        with mock.patch.object(
            control,
            "_original_private_runtime_config",
            side_effect=lambda _paths, text, _store: text,
        ):
            result = control.private_runtime_config(mock.sentinel.paths, source, {})

        self.assertNotIn("route-exclude-address", result)
        self.assertIn("strict-route: true", result)


if __name__ == "__main__":
    unittest.main()
