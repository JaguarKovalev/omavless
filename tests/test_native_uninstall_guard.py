# SPDX-License-Identifier: MIT
"""Offline legacy-uninstall admission tests; every host mutation is stubbed."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "uninstall.sh"


class NativeUninstallGuardTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="omavless-uninstall-guard-")
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.home = self.base / "home"
        self.home.mkdir()
        self.bin = self.base / "bin"
        self.bin.mkdir()
        self.trace = self.base / "trace"
        self.state = self.home / ".local/state/omavless"
        self.state.mkdir(parents=True, mode=0o700)
        self.data = self.home / ".config/omavless"
        self.data.mkdir(parents=True)
        self.sentinel = self.data / "synthetic-profile"
        self.sentinel.write_text("synthetic private sentinel")
        self.env = {"HOME": str(self.home), "PATH": str(self.bin),
                    "UNINSTALL_TEST_TRACE": str(self.trace)}
        # No real systemctl, rm, daemon-reload, or native executable is reachable.
        for name in ("systemctl", "rm", "omavless"):
            path = self.bin / name
            path.write_text("#!/bin/sh\n"
                            f"printf '%s\\n' '{name}' >> \"$UNINSTALL_TEST_TRACE\"\n"
                            'case " $* " in *" is-active "*|*" is-enabled "*) exit 1 ;; esac\n'
                            "exit 0\n")
            path.chmod(0o700)

    def invoke(self, *arguments):
        return subprocess.run(["/bin/bash", str(SCRIPT), *arguments], env=self.env,
                              capture_output=True, text=True, timeout=5, check=False)

    def assert_refused(self):
        for arguments in ((), ("--purge",)):
            with self.subTest(arguments=arguments):
                result = self.invoke(*arguments)
                self.assertEqual(result.returncode, 1)
                self.assertIn("Legacy uninstall refused", result.stderr)
                self.assertEqual(result.stdout, "")
                self.assertFalse(self.trace.exists(), "refusal must precede every command")
                self.assertTrue(self.sentinel.is_file())
                self.assertNotIn(str(self.base), result.stderr)
                self.assertNotIn("private-token", result.stderr)

    def test_all_marker_contents_refuse_even_when_runtime_stopped(self):
        marker = self.state / "ownership.json"
        for content in ('{"phase":"rust"}', '{"phase":"legacy"}',
                        '{"phase":"cutoverPreparing"}', '{"phase":"rollbackPreparing"}',
                        "private-token malformed", ""):
            with self.subTest(content=content):
                marker.write_text(content)
                self.assert_refused()

    def test_frontend_selector_alone_refuses(self):
        (self.state / "frontend-bridge.target").write_text("private-token")
        self.assert_refused()

    def test_marker_wrong_type_and_dangling_link_refuse(self):
        marker = self.state / "ownership.json"
        marker.mkdir()
        self.assert_refused()
        marker.rmdir()
        marker.symlink_to(self.base / "absent")
        self.assert_refused()

    def test_selector_dangling_link_refuses(self):
        (self.state / "frontend-bridge.target").symlink_to(self.base / "absent")
        self.assert_refused()

    def test_state_directory_symlink_and_dangling_link_refuse(self):
        self.state.rmdir()
        target = self.base / "empty-state"
        target.mkdir()
        self.state.symlink_to(target, target_is_directory=True)
        self.assert_refused()
        self.state.unlink()
        self.state.symlink_to(self.base / "absent", target_is_directory=True)
        self.assert_refused()

    def test_state_directory_wrong_type_refuses(self):
        self.state.rmdir()
        self.state.write_text("private-token")
        self.assert_refused()

    def test_ancestor_symlink_refuses(self):
        link = self.base / "state-link"
        link.symlink_to(self.home / ".local", target_is_directory=True)
        self.env["XDG_STATE_HOME"] = str(link / "state")
        self.assert_refused()

    @unittest.skipIf(os.geteuid() == 0, "root bypasses directory permission checks")
    def test_unsearchable_or_unreadable_ancestor_refuses(self):
        parent = self.home / ".local"
        for mode in (0o600, 0o100):
            parent.chmod(mode)
            try:
                self.assert_refused()
            finally:
                parent.chmod(0o700)

    def test_invalid_state_roots_refuse_before_missing_ancestor_shortcut(self):
        for value in ("", "relative", str(self.base / "absent") + "/../state",
                      str(self.base) + "/./state", "/" + "x" * 4096,
                      str(self.base) + "/private-token\nstate"):
            with self.subTest(value=value):
                self.env["XDG_STATE_HOME"] = value
                self.assert_refused()

    def test_invalid_or_missing_home_refuses_even_with_explicit_state_root(self):
        self.env["XDG_STATE_HOME"] = str(self.base / "absent-state")
        for value in ("", "relative", "/", str(self.base) + "/../home"):
            self.env["HOME"] = value
            self.assert_refused()
        self.env.pop("HOME")
        self.assert_refused()

    def test_explicit_xdg_state_marker_is_checked(self):
        custom = self.base / "custom state/omavless"
        custom.mkdir(parents=True)
        (custom / "ownership.json").write_text("private-token")
        self.env["XDG_STATE_HOME"] = str(custom.parent)
        self.assert_refused()

    def test_path_metacharacters_are_data_not_commands(self):
        custom = self.base / "$(private-token); state/omavless"
        custom.mkdir(parents=True)
        (custom / "ownership.json").write_text("synthetic")
        self.env["XDG_STATE_HOME"] = str(custom.parent)
        self.assert_refused()

    def test_proven_absence_preserves_stubbed_legacy_flow(self):
        for arguments in ((), ("--purge",)):
            result = self.invoke(*arguments)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(self.trace.read_text().splitlines(),
                             ["systemctl", "systemctl", "systemctl", "rm", "systemctl"]
                             + (["rm"] if arguments else []))
            self.trace.unlink()
            self.assertTrue(self.sentinel.is_file(), "rm is only a stub")

    def test_absent_state_ancestor_preserves_stubbed_legacy_flow(self):
        self.env["XDG_STATE_HOME"] = str(self.base / "not-created/state")
        result = self.invoke()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(self.trace.exists())
        self.assertTrue(self.sentinel.is_file())

    def test_invalid_option_has_no_effect(self):
        result = self.invoke("--private-token")
        self.assertEqual(result.returncode, 2)
        self.assertFalse(self.trace.exists())
        self.assertNotIn("private-token", result.stderr)


if __name__ == "__main__":
    unittest.main()
