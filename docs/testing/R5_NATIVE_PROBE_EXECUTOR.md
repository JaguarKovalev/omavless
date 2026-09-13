# R5 detached isolated-probe execution

Local-only continuation of the pure plan and authenticated controller adapters.
`probe_executor::execute_plan` is not registered in daemon/CLI/QML; it does not
read the private store, resolve a hostname, admit a job or publish a cache.
The existing installed product is unchanged by this checkpoint.

## Ownership and effects

The enclosing runtime supplies a retained `AuxiliaryLease`, canonical `ProbePlan`,
fixed discovered core path, private scratch parent and absolute job deadline.
The executor never spawns a raw `Child` or accepts caller-provided shell/config
text. Every chunk goes through the lease's parent-owned fixed-argv spawn and
PID/process-start/config/controller proof. Work and cleanup must happen outside
the native-owner mutex.

Each chunk allocates one exclusive 0700 directory with bounded collision retries,
writes its plan-owned credential-bearing configuration atomically as 0600, and
keeps controller/data/config below that directory. No raw core log is captured.
The plan has no TUN, inbound or TCP controller. A socket needs authenticated
same-child normalization before fixed controller requests. Configured readiness
requires the exact group/alias set, not merely a live `/version`.

Exactly three whole-group public HTTPS rounds run per chunk. HTTP responses use
the existing plan collector and Python-compatible per-profile median. Progress
callbacks expose only accepted chunk/round indexes; a scheduler must separately
fence publication and map final positional results to its captured private IDs.
The executor returns only after each temporary core is stopped and reaped.

Cancellation/deadline is checked before effects, during bounded controller I/O,
after I/O and before returning results. Urgent lease revocation prevents another
round or child. Progress-callback unwind triggers RAII child cleanup. Cleanup
failure overrides a success/error outcome with a fixed manual-recovery class;
scratch is retained if child cleanup is uncertain. Scratch replacement refuses
recursive cleanup instead of deleting a different inode. Normal successful
execution leaves the reservation held so the owner can finish publication before
releasing it.

The whole supplied budget cannot exceed 30 minutes; readiness takes at most 10
seconds per chunk and each group round at most 10 seconds, all shortened by the
same absolute job deadline. Child drain has the separate bounded auxiliary-owner
cleanup budget so a timed-out operation still attempts safe cleanup.

## Reference and test boundary

Python `run_mihomo_probe` remains the executable migration oracle. Rendering,
fixed public URL rounds, response merge and aggregation reuse the accepted
153 plan differential / 14 response-merge cases. Intentional strengthening:
bounded chunks and response/deadline/cancellation limits, explicit child lease
ownership, no persisted raw core log and checked cleanup failure.

Deterministic executor tests run an actual parent-owned test-executable child
serving a real private Unix socket. The fixed test wrapper is not a production
helper and does not use Python or an external network fixture. Cases cover
three-round success, sequential chunks, silent-round cancellation, malformed
response cleanup/error safety, callback panic, urgent owner quiescence, empty
and pre-cancelled work, private scratch modes, unsafe parent and inode replacement.
Synthetic child responses are not provider interoperability evidence.

Remaining gates before activation: pinned resolver policy, admitted snapshot and
revision-fenced cache/result publication, one-owner scheduler/observer integration,
exact installed active-VPN plus urgent disconnect/child cleanup acceptance, and
UI bridge. Python cannot be removed and R5/R6 cannot be called complete yet.

## Narrow crash-scratch reconciliation prerequisite

Scratch allocation now includes a fixed 0600 `.omavless-probe-owner` marker,
containing only the format magic and creating daemon PID. The startup-only
`cleanup_orphans` helper is deliberately not registered here. Its caller must
hold the canonical runtime owner lock and committed native migration lease,
before starting a successor core. It supplies complete strict process/TUN
absence proof, checked twice only when orphan candidates exist.

The helper additionally requires the recorded daemon PID to be absent (PID reuse
fails closed), a same-user 0700 parent/directory, exact bounded generated names,
the valid marker and no responsive controller. It examines at most 512 parent
entries and 32 candidate directories. Only four fixed members are recognized:
marker, 0600 config up to the plan's 4-MiB bound, private stale controller socket,
and cache.db up to 32 MiB. Unknown files, subdirectories, symlinks, hard-linked
files, unsafe modes, oversized files, bad/missing markers or incomplete host
proof refuse the entire preflight with `CleanupRequired`.

After complete preflight, inode checks and held-directory descriptors protect
fixed-name `unlinkat` operations; the helper never recursively deletes a
`probe-*` prefix or the runtime directory/owner lock. The marker is removed last
so a cleanup interruption before that point retains ownership proof for retry.
An interruption after marker removal but before removing the empty directory
is conservatively manual recovery, not blind deletion of an unmarked directory.

A controlled no-TUN Mihomo 1.19.30 linux/arm64 startup (no HTTPS group round or
private fixture) observed exactly config.yaml (0600), controller.sock (0666)
and cache.db (0644, 16 KiB) under the 0700 scratch directory. The test's shell
umask was 0022; the packaged service's 0077 narrows creation modes. Cleanup
therefore accepts cache 0600/0644 and socket 0600/0666 only below that strictly
private parent. The exact synthetic child was stopped/reaped. No installed
plugin/runtime state was changed by that isolated inventory.

Four deterministic cleanup tests cover preserved unrelated owner.lock, dead
versus live/refused sockets and owner PIDs, host-proof refusal, whole-set
preflight, unknown/missing marker, symlink/mode/size rejection, count bounds and
replacement races. The production startup seam and actual packaged daemon
SIGKILL/restart gate remain the enclosing integration's responsibility.
