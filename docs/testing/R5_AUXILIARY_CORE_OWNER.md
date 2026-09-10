# R5 disposable probe-core ownership foundation

Local-only continuation of the pure subscription probe plan (#233). No runtime
registration, installed plugin change or Python retirement is claimed here.

The Python reference `run_mihomo_probe` owns a temporary Mihomo process while
the requested tunnel can remain connected. Native host inventories currently
count every visible Mihomo; simply launching a second child would violate that
owner's single-tunnel-core checks. Ignoring processes by name is not a remedy.

`auxiliary_core` provides one explicitly reserved parent-owned child slot. It
reuses `OwnedCore`'s fixed argv, process-group cleanup and unreaped PID pinning.
The lease additionally verifies process start identity and parent, exact private
configuration inode/content and directory, and the private controller's peer
PID/UID and inode once present. It accepts no numeric PID adoption. A failed
proof is an error, never a fabricated empty inventory.

Quiescence revokes the old lease before draining; no new lease can spawn until
the lifecycle guard is released. Child termination/reaping happens outside the
slot mutex, and **the caller must not hold the runtime owner mutex**. Failed
cleanup retains the child and blocks future admission. Old lease completion
cannot revoke its successor. Separate chunk cleanup retains the job reservation.

Eight deterministic tests use disposable synthetic processes and cover exclusive
admission, cancellation before spawn, stop/reap, sequential chunks, stale
completion, modified/replaced config, unsafe files, unowned controller, process
identity mismatch, concurrent cleanup and poisoned/failed state. Strict clippy
passes. This is a pre-integration primitive: not evidence of installed private
VPN/probe coexistence.

Before enabling the operation: bind this slot to the native host and scheduler;
quiesce outside owner locks before lifecycle effects; keep actual visible-core
counts truthful with an explicit auxiliary count; enforce immutable result
snapshot/revision fences; run active-VPN cancellation/disconnect/restart gates.
Latency results remain bounded per-instance memory, like the Python QML cache,
not new writes to the canonical credential store.
