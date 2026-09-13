# R6 attended installed Python-absence lifecycle gate

Status: **installed Python-unavailable lifecycle executed; lifecycle/cleanup
PASS, HTTPS probe FAIL (`probe_timeout`)**. This is not an overall network PASS
or an assertion that R6 is complete.

The installed disposable-account QML/domain matrix already passes with Python
inaccessible. Its account has no normal graphical polkit session. Reusing the
owner's real graphical session for one remaining installed lifecycle check
preserves the existing host authorization path. Restarting the owner's user
manager merely to change its mount namespace would terminate that session.

## Explicit VM-wide test scope

`tests/installed_python_mask.py` and its fixed-purpose Bash guardian are optional
developer tools, never installed application helpers. This alternative is
**VM-wide**, not application-scoped: during the check, new execution of the
system Python interpreter and its aliases is denied for other VM applications
too. Do not use this on a shared/production host or while unrelated Python jobs
or package operations are running. Announce that scope before running it.

The already loaded Python observer imports its modules first. It then requests
ordinary terminal sudo authorization for a temporary bind mount of a root-owned
empty mode-000 file over the exact packaged `/usr/bin/python3.14`. The file is
neither uninstalled nor overwritten. Guards verify the target's package,
identity, hash, ownership, permissions, existing-mount absence and all detected
Python aliases. Other running Python jobs refuse the test. Actual execution
negative controls must fail; merely modifying PATH is not enough.

A separate root-owned Bash watchdog starts **before** the bind mount, survives
the observer/guardian's termination, and restores the exact owned mount on
guardian death or after 15 minutes. Normal context exit/exception also restores
it. Restoration checks the unique mask inode and mount ID, then the original
executable's identity and digest. It does not touch the VPN, terminate an
authentication dialog, or impose a deadline on human authorization. Expiry
invalidates the Python-absence result. Failed restoration retains a fixed
root-owned recovery record instead of deleting unknown state.

## Attended execution

Only after package recovery is complete, startup is Off, the plugin is idle and
the runtime is cleanly disconnected, use a normal visible terminal:

```bash
/usr/bin/python3 tests/installed_native_acceptance.py \
  --run --authorize-socket-inspection \
  --python-unavailable --authorize-vmwide-python-mask
```

Both mask opt-ins are required. Headless invocation refuses before masking.
The ordinary [per-action human barrier](HOST_AUTHORIZATION_ACCEPTANCE.md) stays
in force: no scripted `ready`/`settled`, new policy rules or authorization bypass.
The mask is checked immediately after the pre-action human wait and again after
the post-action wait. If it expired, no further transition runs under a false
absence claim. An unsettled authorization prevents automatic VPN cleanup, while
the independent interpreter restoration still executes.

This reuses the actual installed fixture selection, Full VPN/core/TUN/controller
checks, bounded HTTPS probe and restoration checks. No failed HTTPS result is
changed into PASS, and a timeout does not diagnose the provider, VM or DNS.
Record useful lifecycle evidence separately if the overall network gate fails.
Final Python-unavailable PASS is emitted only after interpreter restoration.

If both independent guardians were forcibly killed, inspect the fixed target
mount and root-owned record under `/run/omavless-r6-python-mask`. Only unmount an
exactly verified owned mask; never delete interpreter files or remove unknown
mounts. A VM reboot removes this transient mount. Do not reboot automatically
while VPN or authorization recovery remains unresolved.

Deterministic policy tests exercise refusal, post-wait mask guards, negative
controls, exception/expiry cleanup and owned-mount restoration using synthetic
accessors. They never run sudo, mount, package, service or VPN effects.

## Installed execution — 2026-09-12

Try Omarchy ARM64 VM, tool `b878bc4`, native package source
`7b75b747883d66a05f1f2b321d42f194c6040b5c`, executable SHA-256
`fe04fba32d4e135d56e929350cd0296d3f89168e06d6ebba9a6de089aec4b786`.
The exact existing installed gate ran in a visible real terminal, with normal
human pre/post authorization. The mask context entered only after actual kernel
refusal of interpreter execution; each mutation checked it after human waiting.

| Check | Actual result |
| --- | --- |
| Real installed Full VPN transition without executable Python | PASS, 141 ms local command time, excluding human authorization |
| Native service ownership, single Mihomo and single TUN | PASS |
| Owned private Unix controller, actual global mode | PASS |
| PID/inode TCP attribution | PASS; only expected loopback proxy/system-TUN forwarder, no TCP controller config |
| Fixed bounded HTTPS probe bound to the TUN | FAIL: `probe_timeout`; successful TUN-carried HTTPS not demonstrated |
| Finally Disconnect, Routing restoration, fresh no-recovery/empty core/TUN/auxiliary facts | PASS; human barriers settled |
| Interpreter identity/hash/execution restored, temporary guardian directory removed | PASS |

The installed gate exited with `https_probe_failed` after successful cleanup.
It did not emit an overall Python-free network PASS. Context restoration
preserved the original error (not a mask-expiry/restoration error), and the
outer observer independently verified restored interpreter bytes and execution.
The useful no-Python lifecycle evidence is distinct from the failed network
probe and from still-unrun Last/pinned fresh-login acceptance. Do not diagnose
DNS, the provider or virtualization from this timeout alone. No retries or
production timeout/security changes were made to obtain a green result.

The owner explicitly kept the existing DNS/provider investigation separate from
Rust retirement. This record closes the observed no-Python lifecycle/guarded
cleanup path, not internet interoperability or DNS/route-restoration correctness.
Raw fixed-result logs remain outside Git; private fixture input was not printed
or committed.
