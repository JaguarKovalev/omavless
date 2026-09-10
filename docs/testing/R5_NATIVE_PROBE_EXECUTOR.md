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
