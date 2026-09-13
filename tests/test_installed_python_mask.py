"""Effect-free tests: never sudo, mount or modify an interpreter."""
import errno
import hashlib
import importlib.util
import io
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import Mock, patch

SPEC = importlib.util.spec_from_file_location("installed_python_mask", Path(__file__).with_name("installed_python_mask.py"))
mask = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(mask)
SCRIPT = Path(__file__).with_name("installed_python_mask.sh")


class PythonMaskPolicyTests(unittest.TestCase):
    def test_default_and_headless_context_refuse_before_any_effect(self):
        for enabled in (False, True):
            with patch.object(mask.subprocess, "Popen", side_effect=AssertionError("effect")), patch.object(mask.sys, "stdin", io.StringIO()):
                with self.assertRaises(mask.MaskError):
                    with mask.InstalledPythonMask(authorize_vmwide=enabled):
                        self.fail("entered mask")

    def test_root_script_no_arguments_is_effect_free_and_syntax_valid(self):
        subprocess.run(["bash", "-n", str(SCRIPT)], check=True)
        result = subprocess.run(["bash", str(SCRIPT)], capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 2)
        self.assertEqual(result.stdout, b"PYTHON_MASK_REFUSED\n")
        self.assertEqual(result.stderr, b"")

    def test_root_guards_and_independent_watchdog_precede_mount(self):
        source = SCRIPT.read_text()
        mount = source.index('  mount --bind "$session/mask" "$target"')
        for guard in ("$EUID == 0", '/proc/1/ns/mnt', '/proc/1/ns/user',
                      'pacman -Qqo "$target"', '/var/lib/pacman/db.lck',
                      'setsid /bin/bash "$session/guardian.sh" --watchdog',
                      '[[ -f $session/watchdog-ready ]] || exit 1'):
            self.assertLess(source.index(guard), mount)
        self.assertIn('SECONDS-started >= 900', source)
        self.assertIn('[[ ! -e $session/restored && -z $(mount_id) ]]', source)
        self.assertIn('[[ $release == release ]] || exit 1', source)
        self.assertIn('trap cleanup EXIT', source)
        self.assertNotRegex(source, r'\b(?:pkexec|resolvectl|systemctl|setcap)\b')
        self.assertNotIn('rm -', source)
        self.assertNotIn('eval ', source)

    def test_other_python_jobs_are_counted_without_names_or_arguments(self):
        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            for pid, name in ((12, b"python3.14\n"), (13, b"python\n"), (14, b"unrelated\n")):
                (root / str(pid)).mkdir()
                (root / str(pid) / "comm").write_bytes(name)
            self.assertEqual(mask.other_python_jobs(root, 12), 1)
            (root / "13" / "comm").unlink()
            self.assertEqual(mask.other_python_jobs(root, 12), 0)

    def test_mount_query_error_cannot_look_like_absent_mount(self):
        source = SCRIPT.read_text()
        function = source[source.index("mount_id() {"):source.index("\nidentity()")]
        result = subprocess.run(["bash", "-c", 'target=/synthetic/python\nawk() { return 1; }\n' + function + '\n[[ -z $(mount_id) ]]\n'], capture_output=True, timeout=5)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stderr, b"")

    def test_negative_controls_require_failed_execution_and_do_not_echo_errors(self):
        def success(*args, **kwargs):
            self.assertEqual(args[0][1:], ["-c", "raise SystemExit(0)"])
            self.assertIs(kwargs["stdout"], subprocess.DEVNULL)
            return subprocess.CompletedProcess(args, 0)
        self.assertFalse(mask.denied_execution(Path("/synthetic/python"), success))
        for returncode in (1, 126, 127, -9):
            self.assertFalse(mask.denied_execution(Path("/synthetic/python"), lambda *a, **k: subprocess.CompletedProcess(a, returncode)))
        for code in (errno.EACCES, errno.EPERM, errno.ENOEXEC, errno.ENOENT, errno.EIO, errno.EAGAIN):
            def denied(*args, **kwargs):
                raise OSError(code, "private-token")
            self.assertEqual(mask.denied_execution(Path("/synthetic/python"), denied), code in (errno.EACCES, errno.EPERM, errno.ENOEXEC))
        self.assertNotIn("private-token", str(mask.MaskError("private-token")))

    def test_restore_requires_exact_original_identity_and_digest(self):
        with tempfile.TemporaryDirectory() as raw:
            target = Path(raw) / "synthetic-interpreter"
            target.write_bytes(b"original")
            original = mask.interpreter_identity(target)
            digest = hashlib.sha256(target.read_bytes()).hexdigest()
            for alteration in ("none", "identity", "digest"):
                value = mask.InstalledPythonMask()
                process = Mock(stdin=io.BytesIO(), returncode=0)
                value.process = process
                value.original = original if alteration != "identity" else (0, *original[1:])
                value.digest = digest if alteration != "digest" else "0" * 64
                with patch.object(mask, "TARGET", target), patch.object(mask, "guardian_output", return_value=b"PYTHON_MASK_RESTORED\n"):
                    if alteration == "none":
                        value._restore()
                    else:
                        with self.assertRaisesRegex(mask.MaskError, "python_mask_recovery_required"):
                            value._restore()
                self.assertIsNone(process.stdin)
                process.kill.assert_not_called()
                process.terminate.assert_not_called()

    def test_restore_timeout_keeps_guardian_alive_and_retains_handle(self):
        value = mask.InstalledPythonMask()
        process = Mock(stdin=io.BytesIO())
        value.process = process
        with patch.object(mask, "guardian_output", side_effect=subprocess.TimeoutExpired("synthetic", 20)):
            with self.assertRaisesRegex(mask.MaskError, "python_mask_recovery_required"):
                value._restore()
        self.assertIsNone(process.stdin)
        self.assertIs(value.process, process)
        process.kill.assert_not_called()
        process.terminate.assert_not_called()

    def test_stdout_is_drained_with_constant_capture_bound(self):
        # Harmless child output only: no sudo/mount/interpreter mutation.
        for count in (0, 20, 160, 161, 65536):
            with subprocess.Popen(["/bin/bash", "-c", 'printf "%*s" "$1" ""', "fixture", str(count)], stdout=subprocess.PIPE) as process:
                output = mask.guardian_output(process, timeout=5)
            self.assertEqual(len(output), min(count, 161))
            self.assertEqual(process.returncode, 0)

    def test_restore_rejects_overflow_or_unexpected_public_output(self):
        with tempfile.TemporaryDirectory() as raw:
            target = Path(raw) / "synthetic-interpreter"
            target.write_bytes(b"original")
            for output in (b"x" * 161, b"private-token\n"):
                value = mask.InstalledPythonMask()
                value.process = Mock(stdin=io.BytesIO(), returncode=0)
                value.original = mask.interpreter_identity(target)
                value.digest = hashlib.sha256(target.read_bytes()).hexdigest()
                with patch.object(mask, "TARGET", target), patch.object(mask, "guardian_output", return_value=output):
                    with self.assertRaisesRegex(mask.MaskError, "^python_mask_unavailable$"):
                        value._restore()

    def test_exit_restores_even_when_authentication_or_callback_failed(self):
        value = mask.InstalledPythonMask()
        with patch.object(value, "_restore") as restore, patch.object(value, "require_active", side_effect=AssertionError):
            self.assertFalse(value.__exit__(RuntimeError, RuntimeError("unsettled"), None))
            restore.assert_called_once()

    def test_expiry_cannot_publish_success_and_restoration_runs_first(self):
        value = mask.InstalledPythonMask()
        calls = []
        with patch.object(value, "_restore", side_effect=lambda: calls.append("restored")), patch.object(value, "require_active", side_effect=mask.MaskError("python_mask_guardian_expired")):
            with self.assertRaisesRegex(mask.MaskError, "python_mask_guardian_expired"):
                value.__exit__(None, None, None)
        self.assertEqual(calls, ["restored"])

    def restore_fixture(self, mode):
        # Execute ONLY the real restore function, overriding every host accessor
        # and umount with harmless synthetic functions. No root/mount syscall.
        source = SCRIPT.read_text()
        function = source[source.index("restore() ("):source.index('\n\nif [[ $1 == --watchdog ]]')]
        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            (root / "original").write_text("1:2:0:755:1 " + "a" * 64 + "\n")
            (root / "mounted").touch()
            if mode != "receipt_window":
                (root / "mount-id").write_text("123\n")
            body = r'''
set -euo pipefail
session=$1
mode=$2
target=/synthetic/python
safe_session() { return 0; }
mount_id() {
  if [[ -e $session/mounted ]]; then
    if [[ $mode == wrong_id ]]; then printf 124; else printf 123; fi
  fi
}
identity() {
  if [[ $1 == "$session/mask" ]]; then printf 3:4:0:0:0
  elif [[ -e $session/mounted ]]; then
    if [[ $mode == wrong_inode ]]; then printf 9:9:0:0:0; else printf 3:4:0:0:0; fi
  else printf 1:2:0:755:1; fi
}
sha256sum() { printf '%064d  synthetic\n' 0 | tr 0 a; }
umount() { [[ $1 == -- && $2 == /synthetic/python ]]; unlink "$session/mounted"; : >"$session/unmounted"; }
'''
            result = subprocess.run(["bash", "-c", body + "\n" + function + "\nrestore\n", "fixture", str(root), mode], capture_output=True, timeout=5)
            return result.returncode, (root / "unmounted").exists(), (root / "restored").exists()

    def test_restore_unmounts_only_matching_owned_mount_and_inode(self):
        self.assertEqual(self.restore_fixture("matching"), (0, True, True))
        for mode in ("wrong_id", "wrong_inode"):
            code, unmounted, restored = self.restore_fixture(mode)
            self.assertNotEqual(code, 0)
            self.assertFalse(unmounted)
            self.assertFalse(restored)

    def test_guardian_death_between_mount_and_receipt_is_recoverable(self):
        self.assertEqual(self.restore_fixture("receipt_window"), (0, True, True))


if __name__ == "__main__":
    unittest.main()
