# R5 packaged native login integration — local candidate

This connects the existing planner, receipt transaction, strict empty-host
observer and isolated validation adapter. It is not an automatic migration,
package enablement hook, or a claim that R5/R6 acceptance is complete. No installed
units or private store were changed while implementing this checkpoint.

## Fixed trigger and ownership

`omavless-runtime.service` requires and is ordered after the separate
`omavless-login-prepare.service`. The latter is a fixed `Type=oneshot`,
`RemainAfterExit=yes` service: it is not attached to each runtime restart.
Both preserve the same private runtime directory, so stopping one unit does not
unlink the cooperating owner's lock inode. It survives service stops, not
user-runtime-directory teardown; no recursive cleanup is introduced.

The only commands are argument-free `login-condition` and `login-prepare`.
They accept no user-supplied epoch, path, unit, executable or shell fragment.
The epoch is the root system manager's `user@<current UID>.service` InvocationID,
not the oneshot's per-start InvocationID. Fixed bounded read-only systemctl
queries use a cleared environment. The root manager's fixed unit/User/MainPID
assignment, root-owned system unit, process UID, actual parent PID, and the
fixed oneshot's MainPID/ControlPID, state and
InvocationID must agree. Environment values only corroborate this identity.
The manager may be activating during initial login. An ordinary terminal
process cannot authorize a login by copying environment variables.
The manager's `/proc/PID/exe` is intentionally not a prerequisite: normal
non-dumpable systemd managers deny that read. The read-only installed identity
test caught this availability issue without weakening ptrace/OS policy.

The running executable must be the root-owned packaged binary. Both effective
units must be the exact root-owned packaged bytes, with no drop-ins or pending
daemon reload. Current process, service and user-manager XDG roots must agree
through the existing activation environment checks. Custom home overrides
refuse; there is no generic fallback when identity discovery fails.

`ExecCondition` skips only a validated Legacy or valid CutoverPreparing marker.
Missing/corrupt ownership, malformed queries and RollbackPreparing fail with
exit 255, not the condition-skip range. Existing deliberate cutover candidate
construction remains separate: it cannot advertise startup configuration and
does not require a fresh-login receipt. After promotion, an ordinary controlled
runtime restart must pass the packaged login path.

## Publication and restart

Preparation acquires owner then migration locks through `consume_login`, proves
the host empty, validates exact store/template/desired snapshots and retains
the existing pending → desired → consumed publication ordering. Enabled login
uses the selected native core with the existing NoNewPrivs/file-capability
preconditions and the isolated offline bwrap validator. Disabled login needs no
core validation and publishes disconnected intent. Neither path provisions
missing rule caches, downloads anything, or repairs unresolved legacy policy.

Ordinary committed-native production construction now requires a consumed
receipt for the exact ownership generation and current manager epoch **before**
reconciliation. Absent, stale-manager, pending or corrupt receipts block; direct
`daemon` invocation cannot bypass first-login application. Retrying an already
consumed epoch preserves current desired state, so explicit Disconnect followed
by runtime restart does not reconnect from saved login preferences. The pure
injected constructor retains its original barrier for deterministic tests.

No default is guessed for unresolved imported startup settings. The accepted
production cutover preflight already rejects unconfigured or enabled legacy
startup (`production_cutover::observe`); current native migration therefore
enters with explicitly configured Off. An unresolved legacy installation must
complete that existing migration prerequisite before native activation. Do not
delete a receipt or edit desired state to bypass a failed login transaction.

## Settings and package setup

`startup.configure` uses the existing canonical request, revision/instance fence,
atomic private-store write and replay contract. It does not alter current desired
connection intent. Generic plugin actions reach the same dispatcher and gate.
Production capabilities advertise it only after exact packaged receipt startup
and when the canonical runtime unit was observed persistently enabled. Candidate
cutover owners never advertise it; injected tests admit it explicitly.

Unit enablement is explicit package setup, not an effect of saving preferences:
enable the canonical runtime user service through the documented native package
activation workflow after migration preflight. Off means the enabled runtime
starts disconnected; it does not disable the daemon. Package installation itself
does not enable/start services. The enabled-unit observation is a startup snapshot:
external service disable/override requires restart and capability re-evaluation;
saved preferences and capability discovery do not prove future session startup.
There is no systemctl mutation, arbitrary shell or privileged API exposed to UI.

`bubblewrap` is an ordinary Arch package dependency because production enabled
startup validation now requires it. Namespace restrictions remain fail-closed.
Only exact bundled templates and complete bounded cached resources are accepted;
custom templates, unavailable caches and unsupported core executable formats
remain explicit validation limitations. Offline success is not TUN readiness,
DNS correctness or provider reachability. The daemon still uses its canonical
native lifecycle validation and actual-state reconciliation at execution time.

## Validation and outstanding host acceptance

Deterministic gates cover fixed manager/unit process identities and malformed
queries, phase skip policy, spoofed command environments, receipt absence and
epoch/generation mismatch, pending refusal, same-epoch retry preserving current
intent, capability admission, revision-fenced save/replay, private error safety,
and package payload/unit dependency preservation. Existing transaction and
isolated sandbox tests cover lock order, snapshot replacement and cleanup.
The installed manager opt-in is strictly read-only; static `systemd-analyze
--user verify` validates both units without starting them.

Local ARM64 checks on 2026-09-10: full runtime library **497 passed, four
installed opt-ins ignored**, plus **48 integration tests passed**. The separate
installed manager identity opt-in passed after removing the unsupported ptrace
prerequisite. Strict all-target clippy, formatting, payload shell syntax,
diff checks and static verification of both units passed. This run did not
alter the host during the five focused Python packaging cases: four passed,
including actual offline makepkg archive identity/dependency validation, and
the root-only case was skipped. The run did not
repeat the resource-bearing installed Mihomo opt-ins recorded in the preceding
adapter checkpoint and does not count environment-gated tests as new live VPN
evidence.

Before installed acceptance: review and build the combined exact package, reload
the user manager, exercise real condition/oneshot PID identity and dependency
ordering, Off startup, enabled Last/pinned profile, same-manager restart after
explicit Disconnect, new-manager login, missing-cache refusal and deliberate
cutover compatibility. Confirm one owner/core/TUN, Unix-only controllers and
private data unchanged except explicitly requested preferences/intent. Real
login and authentication observations remain host gates, not deterministic-test
claims. This local candidate must not be merged or described as accepted solely
because compilation and injected tests pass.
