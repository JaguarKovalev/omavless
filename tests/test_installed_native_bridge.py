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

    def test_host_driver_has_no_privileged_or_network_state_mutation(self):
        source = SCRIPT.read_text()
        code = "\n".join(line for line in source.splitlines()
                         if not line.lstrip().startswith("#"))
        self.assertNotRegex(code, r"\b(?:sudo|pkexec|systemctl|loginctl|useradd|userdel|nft|resolvectl)\b")
        self.assertNotRegex(code, r"/usr/bin/omavless\s+(?:connect|disconnect|quit)\b")
        self.assertIn("sleep 3", source)

    def test_fixture_node_uses_only_two_fixed_guarded_locations(self):
        source = SCRIPT.read_text()
        self.assertIn("fixture_node=/usr/bin/node", source)
        self.assertIn('fixture_node="$HOME/bridge-tools/node"', source)
        self.assertNotIn("command -v node", source)
        for guard in ('! -L $HOME/bridge-tools', '-f $fixture_node',
                      '! -L $fixture_node', '-x $fixture_node',
                      '$(stat -c %u "$fixture_node") == "$EUID"',
                      '$(stat -c %a "$fixture_node") == 700'):
            self.assertIn(guard, source)
        self.assertLess(source.index("stage=fixture_dependencies"),
                        source.index("fixture_node=/usr/bin/node"))
        self.assertIn("BRIDGE DEPENDENCY UNAVAILABLE: trusted Node fixture tool", source)


if __name__ == "__main__":
    unittest.main()
