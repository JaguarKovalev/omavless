#!/usr/bin/python3
# SPDX-License-Identifier: MIT
"""TEST ONLY: attended, temporary VM-wide interpreter-execution restriction.

Load the acceptance driver and all its modules BEFORE entering this context.
The participating interpreter remains mapped; new executions of the installed
Python and its aliases must fail for the real existing CLI/daemon/desktop.

This is not application-only isolation: other VM applications temporarily lose
Python too. Refuse existing other Python jobs; do not run package operations
or start unrelated work while active. The root guardian requires real sudo
authorization in the controlling terminal. Normal polkit and ready/settled
barriers in the acceptance driver remain unchanged.

Use require_active() before each effect and after human waits; publish final
Python-absence acceptance ONLY after the context exits successfully. A 15-minute
watchdog restores Python even if the driver dies, but does not kill the VPN or
put a deadline on an authentication dialog. Expiry invalidates the test.

If BOTH guardian and watchdog are forcibly killed, inspect the fixed mount:
  sudo findmnt --mountpoint /usr/bin/python3.14
Only after confirming its source is /run/omavless-r6-python-mask/mask:
  sudo umount /usr/bin/python3.14
Compare the restored file identity/hash with the root-owned original record.
Do not remove an unknown mount or delete interpreter/package files. Reboot also
removes this transient mount. Root-owned failure records stay for inspection.
"""
import errno
import hashlib
import os
from pathlib import Path
import re
import selectors
import shutil
import stat
import subprocess
import sys
import time

TARGET = Path("/usr/bin/python3.14")
GUARDIAN = Path(__file__).with_suffix(".sh")
INTERPRETER_NAME = re.compile(r"python(?:[0-9]+(?:\.[0-9]+)*)?\Z")


class MaskError(Exception):
    def __init__(self, code="python_mask_unavailable"):
        allowed = {"python_mask_unavailable", "python_mask_refused",
                   "python_mask_not_active", "python_mask_recovery_required",
                   "other_python_workload_running", "python_mask_guardian_expired"}
        super().__init__(code if code in allowed else "python_mask_unavailable")


def interpreter_identity(path):
    meta = path.stat()
    return meta.st_dev, meta.st_ino, meta.st_uid, stat.S_IMODE(meta.st_mode), meta.st_size


def interpreter_aliases(search_path):
    """Before masking, every ordinary interpreter alias must resolve to target."""
    candidates = {TARGET}
    for path in Path("/usr/bin").iterdir():
        if INTERPRETER_NAME.fullmatch(path.name):
            candidates.add(path)
    for name in ("python", "python3"):
        found = shutil.which(name, path=search_path)
        if found:
            candidates.add(Path(found))
    if any(path.resolve(strict=True) != TARGET for path in candidates):
        raise MaskError("python_mask_refused")
    return tuple(sorted(candidates))


def other_python_jobs(proc_root, driver_pid):
    count = 0
    for path in proc_root.iterdir():
        if not path.name.isdecimal() or int(path.name) == driver_pid:
            continue
        try:
            raw = (path / "comm").read_bytes()
        except (FileNotFoundError, ProcessLookupError):
            continue
        except OSError:
            raise MaskError("python_mask_refused") from None
        if len(raw) > 64:
            raise MaskError("python_mask_refused")
        if INTERPRETER_NAME.fullmatch(raw.decode("ascii", errors="replace").strip()):
            count += 1
    return count


def denied_execution(path, run=subprocess.run):
    try:
        run([str(path), "-c", "raise SystemExit(0)"], stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=2)
    except OSError as error:
        # A child exit, missing file, resource exhaustion or unrelated failure
        # does not prove that the kernel refused interpreter execution.
        return error.errno in (errno.EACCES, errno.EPERM, errno.ENOEXEC)
    except subprocess.TimeoutExpired:
        return False
    return False


def guardian_output(process, timeout=20):
    """Drain without unbounded communicate() buffering; retain at most 161 bytes.

    Continue draining excess output so a guardian can finish restoration rather
    than blocking on its pipe. Neither overflow nor timeout kills the guardian.
    """
    deadline = time.monotonic() + timeout
    output = bytearray()
    with selectors.DefaultSelector() as selector:
        selector.register(process.stdout, selectors.EVENT_READ)
        while selector.get_map():
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise subprocess.TimeoutExpired("python-mask-guardian", timeout)
            for key, _ in selector.select(remaining):
                chunk = os.read(key.fd, 4096)
                if not chunk:
                    selector.unregister(key.fileobj)
                else:
                    output.extend(chunk[:max(0, 161 - len(output))])
    process.wait(timeout=max(0, deadline - time.monotonic()))
    return bytes(output)


class InstalledPythonMask:
    """No mount or sudo occurs unless explicitly opted in from a real terminal."""
    def __init__(self, *, authorize_vmwide=False):
        self.authorized = authorize_vmwide
        self.process = None
        self.aliases = ()
        self.original = None
        self.digest = None

    def __enter__(self):
        if (not self.authorized or os.geteuid() == 0
                or not sys.stdin.isatty() or not sys.stderr.isatty()):
            raise MaskError("python_mask_refused")
        try:
            metadata = TARGET.lstat()
            if (not stat.S_ISREG(metadata.st_mode) or metadata.st_uid != 0
                    or stat.S_IMODE(metadata.st_mode) != 0o755 or metadata.st_nlink != 1
                    or interpreter_identity(Path("/proc/self/exe")) != interpreter_identity(TARGET)):
                raise MaskError("python_mask_refused")
            self.aliases = interpreter_aliases(os.environ.get("PATH", ""))
            if other_python_jobs(Path("/proc"), os.getpid()):
                raise MaskError("other_python_workload_running")
            if GUARDIAN.is_symlink() or not GUARDIAN.is_file() or GUARDIAN.stat().st_mode & 0o022:
                raise MaskError("python_mask_refused")
            self.original = interpreter_identity(TARGET)
            self.digest = hashlib.sha256(TARGET.read_bytes()).hexdigest()
            # stderr and /dev/tty remain the visible human authorization route.
            # stdin is exclusively a parent-death lifeline, never passwords.
            self.process = subprocess.Popen(
                ["/usr/bin/sudo", "--", "/bin/bash", str(GUARDIAN), "--run", self.digest, str(os.getpid())],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=None, bufsize=0,
                close_fds=True)
            ready = self.process.stdout.readline(80)
            if ready != b"PYTHON_MASK_READY\n":
                raise MaskError("python_mask_refused")
            self.require_active()
            if not all(denied_execution(path) for path in self.aliases):
                raise MaskError("python_mask_not_active")
            self.require_active()
            return self
        except BaseException as error:
            if self.process is not None:
                self._restore()
            if isinstance(error, (MaskError, KeyboardInterrupt, SystemExit)):
                raise
            raise MaskError("python_mask_unavailable") from None

    def require_active(self):
        """Call immediately before effects, particularly after human waits."""
        if self.process is None or self.process.poll() is not None:
            raise MaskError("python_mask_guardian_expired")
        try:
            meta = TARGET.stat()
            if (not stat.S_ISREG(meta.st_mode) or meta.st_uid != 0 or meta.st_size != 0
                    or stat.S_IMODE(meta.st_mode) != 0
                    or any(shutil.which(name) for name in ("python", "python3"))):
                raise MaskError("python_mask_not_active")
        except OSError:
            raise MaskError("python_mask_not_active") from None

    def _restore(self):
        process = self.process
        if process is None:
            return
        if process.stdin is not None:
            process.stdin.close()  # EOF always restores, including unsettled VPN authorization.
            process.stdin = None
        try:
            output = guardian_output(process)
        except (OSError, subprocess.TimeoutExpired):
            # Never kill the privileged guardian/watchdog or touch the VPN here.
            # Retain the handle for inspection/retry; EOF is already delivered.
            raise MaskError("python_mask_recovery_required") from None
        self.process = None
        try:
            restored = (interpreter_identity(TARGET) == self.original
                        and hashlib.sha256(TARGET.read_bytes()).hexdigest() == self.digest)
        except OSError:
            restored = False
        if not restored:
            raise MaskError("python_mask_recovery_required")
        if (process.returncode != 0 or len(output) > 160
                or output not in (b"PYTHON_MASK_RESTORED\n", b"")):
            raise MaskError("python_mask_unavailable")

    def __exit__(self, kind, value, traceback):
        active_error = None
        if kind is None:
            try:
                self.require_active()
            except MaskError as error:
                active_error = error
        self._restore()
        if active_error is not None:
            raise active_error
        return False
