"""Policy checks for the opt-in installed matrix; no host effects here."""
from pathlib import Path
import os
import subprocess
import unittest


SCRIPT = Path(__file__).with_name("installed_native_domain.sh")


class InstalledDomainPolicyTests(unittest.TestCase):
    def test_shell_syntax(self):
        subprocess.run(["bash", "-n", str(SCRIPT)], check=True)

    def test_no_opt_in_is_effect_free(self):
        result = subprocess.run(["bash", str(SCRIPT)], capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 2)
        self.assertEqual(result.stdout, b"DOMAIN GATE REFUSED: disposable account required\n")
        self.assertEqual(result.stderr, b"")

    def test_explicit_opt_in_still_rejects_wrong_home(self):
        environment = {**os.environ, "HOME": "/synthetic/not-the-disposable-account"}
        result = subprocess.run(["bash", str(SCRIPT), "--run"], env=environment,
                                capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 2)
        self.assertEqual(result.stdout, b"DOMAIN GATE REFUSED: disposable account required\n")
        self.assertEqual(result.stderr, b"")

    def test_fixed_target_guard_precedes_host_access(self):
        source = SCRIPT.read_text()
        for guard in ("$EUID -ne 0", "$(id -un) == omavless-r6-domain",
                      "! ${OMAVLESS_HOME+x}", "${HOME:-} == /home/omavless-r6-domain"):
            self.assertLess(source.index(guard), source.index("scratch=$(mktemp"))
        self.assertIn("/usr/bin/python3.*", source)
        self.assertIn("! command -v python3", source)
        self.assertIn("timeout --kill-after=1s 15s /usr/bin/omavless", source)

    def test_no_connection_privileged_or_fetch_command(self):
        source = SCRIPT.read_text()
        code = "\n".join(line for line in source.splitlines() if not line.lstrip().startswith("#"))
        self.assertNotRegex(code, r"\b(?:sudo|pkexec|systemctl|loginctl|curl|wget|useradd|userdel)\b")
        self.assertNotRegex(code, r"(?m)^\s*(?:action|cli)\s+(?:connect|disconnect|quit|subscription-add|subscription-refresh)\b")
        for proof in ("manualRecoveryRequired==false", "visibleMihomoCount==0",
                      "visibleTunCount==0", "ownedAuxiliaryMihomoCount==0"):
            self.assertIn(proof, source)


if __name__ == "__main__":
    unittest.main()
