"""Effect-free policy gates for the opt-in installed Quickshell bridge matrix."""
from pathlib import Path
import os
import subprocess
import unittest

SCRIPT = Path(__file__).with_name("installed_native_bridge.sh")
QML = SCRIPT.with_suffix(".qml")


class InstalledBridgePolicyTests(unittest.TestCase):
    def test_shell_syntax(self):
        subprocess.run(["bash", "-n", str(SCRIPT)], check=True)

    def test_no_opt_in_refuses_without_effects(self):
        result = subprocess.run(["bash", str(SCRIPT)], capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 2)
        self.assertEqual(result.stdout, b"BRIDGE GATE REFUSED: disposable account required\n")
        self.assertEqual(result.stderr, b"")

    def test_wrong_account_refuses_even_with_opt_in(self):
        result = subprocess.run(["bash", str(SCRIPT), "--run"],
                                env={**os.environ, "HOME": "/synthetic/invalid"},
                                capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 2)
        self.assertEqual(result.stderr, b"")

    def test_guards_and_python_negative_controls(self):
        source = SCRIPT.read_text()
        for guard in ("$EUID -ne 0", "$(id -un) == omavless-r6-domain",
                      "! ${OMAVLESS_HOME+x}", "${HOME:-} == /home/omavless-r6-domain"):
            self.assertLess(source.index(guard), source.index("scratch=$(mktemp"))
        self.assertIn("/usr/bin/python3.*", source)
        self.assertIn("! command -v python3", source)
        self.assertIn('.id == "kdk.omavless"', source)
        self.assertIn("server.listen(0, '127.0.0.1'", source)
        self.assertIn("manualRecoveryRequired==false", source)

    def test_real_service_without_mocked_boundary_or_vpn_actions(self):
        source = QML.read_text()
        self.assertIn("/plugins/kdk.omavless/plugin/Service.qml", source)
        self.assertNotRegex(source, r"service\.(?:backendPath|nativeSnapshot|nativeOwner|nativeObservation)\s*=")
        self.assertNotIn("requestNativeAction(\"connect\"", source)
        self.assertNotIn("requestNativeAction(\"disconnect\"", source)
        for expected in ("startNativeImport(\"file\"", "confirmNativeImport(",
                         '"subscription-add"', '"subscription-update"',
                         '"subscription-refresh"', '"subscription-delete"',
                         "startNativeReportFileExport(", "startNativeFileExport("):
            self.assertIn(expected, source)
        self.assertIn("120000", source)


if __name__ == "__main__":
    unittest.main()
