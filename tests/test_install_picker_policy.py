# SPDX-License-Identifier: MIT
"""Real installer reporting under synthetic HOME/PATH, never a live install.

Only ordinary file utilities are exposed. Omarchy, native ownership and Python
are harmless stubs; no real user store, shell, service, network or auth command
can be reached. GTK fallback is a legacy capability, not native readiness.
"""
from pathlib import Path
import subprocess
import tempfile
import unittest


INSTALLER = Path(__file__).resolve().parents[1] / "install.sh"
MISSING = "File import unavailable — file picker missing."


class InstallPickerPolicyTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="omavless-picker-install-policy-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.bin = self.root / "bin"
        self.bin.mkdir(mode=0o700)
        self.trace = self.root / "trace"
        self.state = self.root / "state"
        self.state.mkdir(mode=0o700)
        self.home = self.root / "home"
        self.target = self.home / ".config/omarchy/plugins/kdk.omavless"
        self.target.mkdir(parents=True, mode=0o700)
        # Existing isolated plugin target skips fresh shell-discovery polling.
        # All copied plugin data remains below this TemporaryDirectory.
        self.env = {
            "HOME": str(self.home), "XDG_STATE_HOME": str(self.state),
            "PATH": str(self.bin), "TEST_TRACE": str(self.trace),
            "TEST_OWNER": "rust", "TEST_OWNER_EXIT": "0", "TEST_GTK_EXIT": "0",
        }
        for utility in ("dirname", "mkdir", "mktemp", "cp", "chmod", "mv", "rm"):
            (self.bin / utility).symlink_to("/usr/bin/" + utility)
        self.stub("omarchy", '''
printf 'omarchy:%s:%s\\n' "$1" "$2" >> "$TEST_TRACE"
[ "$1" = plugin ] && [ "$2" = validate ] && [ "$#" = 3 ] || exit 99
''')
        self.stub("python3", '''
printf 'python-gtk\\n' >> "$TEST_TRACE"
[ "$#" = 2 ] && [ "$1" = -c ] || exit 98
printf 'synthetic-private-python-error\\n' >&2
exit "$TEST_GTK_EXIT"
''')
        self.native()

    def stub(self, name, body):
        path = self.bin / name
        path.write_text("#!/bin/sh\n" + body)
        path.chmod(0o700)

    def native(self):
        self.stub("omavless", '''
printf 'native:%s:%s\\n' "$1" "$2" >> "$TEST_TRACE"
[ "$#" = 2 ] && [ "$1" = plugin ] && [ "$2" = target ] || exit 99
printf '%s\\n' "$TEST_OWNER"
printf 'synthetic-private-owner-error\\n' >&2
exit "$TEST_OWNER_EXIT"
''')

    def run_install(self, *arguments):
        self.trace.unlink(missing_ok=True)
        result = subprocess.run(
            ["/bin/bash", str(INSTALLER), *arguments], env=self.env,
            stdin=subprocess.DEVNULL, capture_output=True, text=True,
            timeout=15, check=False,
        )
        self.assertEqual(result.returncode, 0, "isolated installer failed")
        self.assertEqual(result.stderr, "")
        self.assertTrue((self.target / "manifest.json").is_file())
        self.assertEqual((self.target / "backend.py").is_file(), "--native-only" not in arguments)
        self.assertEqual((self.target / "uninstall.sh").is_file(), "--native-only" not in arguments)
        self.assertNotIn("synthetic-private", result.stdout)
        self.assertIn("no tunnel was started", result.stdout)
        return result.stdout, self.trace.read_text().splitlines()

    def test_native_only_omits_legacy_payload_and_never_probes_python(self):
        (self.target / "backend.py").write_text("synthetic old backend")
        (self.target / "uninstall.sh").write_text("synthetic old remover")
        (self.bin / "python3").unlink()
        output, calls = self.run_install("--native-only")
        self.assert_missing(output)
        self.assertEqual(calls, ["native:plugin:target", "omarchy:plugin:validate", "native:plugin:target"])
        self.assertTrue((self.target / "plugin/Panel.qml").is_file())
        self.assertEqual(list(self.target.rglob("*.py")), [])

    def test_native_only_refuses_unknown_failed_legacy_and_missing_owner_before_writes(self):
        sentinel = self.target / "sentinel"
        sentinel.write_text("preserved")
        for owner, code in (("legacy", "0"), ("unknown", "0"), ("rust\nlegacy", "0"), ("rust", "1")):
            self.env["TEST_OWNER"], self.env["TEST_OWNER_EXIT"] = owner, code
            result = subprocess.run(["/bin/bash", str(INSTALLER), "--native-only"], env=self.env,
                                    capture_output=True, text=True, timeout=15)
            self.assertNotEqual(result.returncode, 0)
            self.assertNotIn("synthetic-private", result.stderr)
            self.assertEqual(list(self.target.iterdir()), [sentinel])
        (self.bin / "omavless").unlink()
        result = subprocess.run(["/bin/bash", str(INSTALLER), "--native-only"], env=self.env,
                                capture_output=True, timeout=15)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(sentinel.read_text(), "preserved")

    def test_native_only_rechecks_owner_after_staging_and_preserves_old_tree(self):
        sentinel = self.target / "sentinel"
        sentinel.write_text("preserved")
        self.stub("omavless", '''
if [ -f "$TEST_TRACE" ]; then printf 'legacy\\n'; else printf 'rust\\n'; fi
''')
        result = subprocess.run(["/bin/bash", str(INSTALLER), "--native-only"], env=self.env,
                                capture_output=True, text=True, timeout=15)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(list(self.target.iterdir()), [sentinel])
        self.assertEqual(list(self.target.parent.glob(".kdk.omavless.install.*")), [])

    def test_install_rejects_unknown_arguments_before_effects(self):
        for arguments in [("--unknown",), ("--native-only", "extra")]:
            result = subprocess.run(["/bin/bash", str(INSTALLER), *arguments], env=self.env,
                                    capture_output=True, text=True, timeout=15)
            self.assertEqual(result.returncode, 2)
            self.assertFalse(self.trace.exists())

    def assert_missing(self, output):
        self.assertIn(MISSING, output)
        self.assertIn("omarchy pkg add zenity", output)
        self.assertIn("Clipboard import remains available", output)

    def test_native_does_not_claim_successful_python_gtk_as_a_picker(self):
        output, calls = self.run_install()
        self.assert_missing(output)
        self.assertEqual(calls, ["omarchy:plugin:validate", "native:plugin:target"])

    def test_native_reporting_works_with_python_executable_absent(self):
        (self.bin / "python3").unlink()
        output, calls = self.run_install()
        self.assert_missing(output)
        self.assertEqual(calls, ["omarchy:plugin:validate", "native:plugin:target"])

    def test_all_supported_pickers_are_reported_without_launch_or_python(self):
        for picker in ("zenity", "kdialog", "yad"):
            self.stub(picker, 'printf "picker-was-launched\\n" >> "$TEST_TRACE"\nexit 99\n')
            try:
                for owner in ("rust", "legacy", "unknown"):
                    with self.subTest(picker=picker, owner=owner):
                        self.env["TEST_OWNER"] = owner
                        output, calls = self.run_install()
                        self.assertNotIn(MISSING, output)
                        self.assertEqual(calls, ["omarchy:plugin:validate"])
            finally:
                (self.bin / picker).unlink()

    def test_legacy_gtk_available_and_unavailable_remain_actionable(self):
        self.env["TEST_OWNER"] = "legacy"
        for code in ("0", "1"):
            with self.subTest(gtk_exit=code):
                self.env["TEST_GTK_EXIT"] = code
                output, calls = self.run_install()
                self.assertEqual(calls, ["omarchy:plugin:validate", "native:plugin:target", "python-gtk"])
                if code == "0":
                    self.assertNotIn(MISSING, output)
                else:
                    self.assert_missing(output)

    def test_unknown_or_failed_canonical_owner_never_probes_python(self):
        for owner, code in (("", "0"), ("unknown", "0"), ("rust\nlegacy", "0"),
                            ("legacy", "1"), ("rust", "73")):
            with self.subTest(owner=owner, code=code):
                self.env["TEST_OWNER"] = owner
                self.env["TEST_OWNER_EXIT"] = code
                output, calls = self.run_install()
                self.assert_missing(output)
                self.assertEqual(calls, ["omarchy:plugin:validate", "native:plugin:target"])

    def test_marketplace_legacy_without_native_or_artifacts_keeps_gtk(self):
        (self.bin / "omavless").unlink()
        for existing_state in (False, True):
            if existing_state:
                (self.state / "omavless").mkdir(mode=0o700)
            output, calls = self.run_install()
            self.assertNotIn(MISSING, output)
            self.assertEqual(calls, ["omarchy:plugin:validate", "python-gtk"])

    def test_missing_native_binary_with_either_marker_never_probes_python(self):
        (self.bin / "omavless").unlink()
        directory = self.state / "omavless"
        directory.mkdir(mode=0o700)
        for name in ("ownership.json", "frontend-bridge.target"):
            for dangling in (False, True):
                marker = directory / name
                if dangling:
                    marker.symlink_to(directory / "missing")
                else:
                    marker.write_text("synthetic ownership artifact: do not parse")
                    marker.chmod(0o600)
                try:
                    output, calls = self.run_install()
                    self.assert_missing(output)
                    self.assertEqual(calls, ["omarchy:plugin:validate"])
                finally:
                    marker.unlink()

    def test_missing_native_unsafe_state_roots_do_not_enable_gtk(self):
        (self.bin / "omavless").unlink()
        link = self.root / "state-alias"
        link.symlink_to(self.state, target_is_directory=True)
        ordinary_file = self.root / "state-file"
        ordinary_file.write_text("synthetic")
        for state in ("", "relative", str(link), str(ordinary_file), str(self.state) + "/../state"):
            with self.subTest(state_kind="synthetic"):
                self.env["XDG_STATE_HOME"] = state
                output, calls = self.run_install()
                self.assert_missing(output)
                self.assertEqual(calls, ["omarchy:plugin:validate"])

    def test_missing_native_default_state_location_remains_legacy_only_if_absent(self):
        (self.bin / "omavless").unlink()
        del self.env["XDG_STATE_HOME"]
        output, calls = self.run_install()
        self.assertNotIn(MISSING, output)
        self.assertIn("python-gtk", calls)
        directory = self.home / ".local/state/omavless"
        directory.mkdir(parents=True, mode=0o700)
        (directory / "ownership.json").write_text("synthetic")
        output, calls = self.run_install()
        self.assert_missing(output)
        self.assertEqual(calls, ["omarchy:plugin:validate"])


if __name__ == "__main__":
    unittest.main()
