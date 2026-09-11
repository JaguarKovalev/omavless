# Native activation without a legacy unit

Local-only R6 composition checkpoint, 2026-09-11, Try Omarchy ARM64.
Exact implementation: `d270fd568432ada56ffb7d2348006a6a8feeb147`.
No installed package/frontend replacement, service transition, authentication,
VPN, DNS or route operation was performed for this checkpoint.

## Boundary

The explicit packaged `omavless cutover activate` now also admits a genuinely
absent `omavless.service`. It still requires the root-owned exact native package,
CLI/user-manager environment agreement, disconnected valid private config with
startup Off, the migration lease, strict repeated process/controller/TUN
inventories and the existing generation-fenced transaction. No new CLI/IPC,
force flag, manual ownership publication or privileged command was added.

Absence is not inferred from a command failure. The fixed bounded installation
query additionally requests LoadState and inactive-process facts. The legacy-only
exception requires exactly one of each property:

| Property | Required value |
| --- | --- |
| LoadState | `not-found` |
| UnitFileState, FragmentPath, DropInPaths | empty |
| NeedDaemonReload | `no` |
| ActiveState | `inactive` |
| MainPID, ExecMainStatus | `0` |
| Result | `success` |

Missing/duplicate/conflicting facts, unknown/masked/error states, nonzero query
exit, invalid encoding, excessive output and timeout fail closed. The native
unit must still be **loaded**, disabled, without drop-ins or pending reload,
and use the exact packaged fragment. An absent native unit is never accepted.
The existing read-only service-state query already recognizes an absent unit's
complete inactive/PID-zero facts; no global observation parser was relaxed.

The transaction re-queries legacy installation facts at its stop boundary and
again during disconnected compensation. Proven absence skips the impossible
legacy stop action and still verifies inactivity. Loaded legacy units retain
the existing stop/wait behavior. Failed/ambiguous observation does not become a
successful stop. Native candidate startup, bootstrap, frontend selection,
commit, compensation and manual-recovery rules remain the accepted transaction.
On fresh-host compensation, “legacy ownership” is the existing inactive default
selector state, **not** a claim that a Python payload exists or was started.
Native-only launchers continue to refuse an absent legacy backend.

## Fresh state-parent composition

`omavless setup initialize` still creates only the two initial config files and
does not activate ownership. Under its migration lease, it now also prepares
the conventional `$HOME/.local` and `$HOME/.local/state` directories when that
is the effective state base. Missing directories are created mode0700; existing
ones must be ordinary, same-user, symlink-free and not group/other-writable.
They are never chmodded or repaired. The OmaVLESS state child and ownership
files are left for the accepted activation path.

A custom `XDG_STATE_HOME` must already exist as a safe same-user directory.
The initializer does not recursively manufacture arbitrary configured roots.
Unsafe/missing custom roots refuse before config creation. As before, interrupted
preparation may leave safe new directories or an exact partial config pair;
retry is create-only, not destructive rollback.

## Evidence and limitations

Synthetic production-host tests exercise the unchanged exact initial store and
default template, absent legacy installation, private candidate socket handshake,
successful Rust commit, and candidate-start failure with verified compensation.
A sentinel proves neither path attempts any legacy start/stop action. These
service/process doubles do not prove a packaged real-user activation.

Strict property tests remove, duplicate and corrupt every absence field and
reject an absent native unit. A fixed-query test rejects exit1 even when its
stdout contains otherwise perfect absence evidence. Actual local read-only
`systemctl --user show` for an intentionally nonexistent synthetic unit returned
exit0 with the expected absent/inactive fields; no unit was installed or changed.

The real executable setup suite checks private default state-parent creation,
exact no-op retry, unsafe-mode preservation, ancestor-symlink refusal and missing
custom-root refusal with no config creation. It runs without usable Python or
host service/privilege entry points in the isolated conformance harness.

Remaining acceptance: build/install the exact package in a clean user environment,
initialize, activate with no legacy unit/payload, finish onboarding/import with
startup Off, and test failure/retry/recovery. Actual login, package upgrade/remove,
support composition, controlled DNS/auth and the full installed Python-unavailable
matrix remain separate gates. This checkpoint does not complete R6 or authorize
publication to main; the installed runtime/frontend remain unchanged.

## Exact-source validation

- Full `cargo test --locked --workspace` with `RUST_TEST_THREADS=2`:
  **944 passed, 10 existing ignored**, no failures.
- The initial default-parallel final-source run failed an existing helper fixture's
  two-second startup-readiness assertion in
  `core::tests::helper_resources_are_drained_even_after_leader_exit_or_term_spawn`.
  Its isolated rerun passed, as did the complete two-thread workspace rerun.
  This is evidence of a load-sensitive test, not proof of a production defect
  or its absence. No runtime/test timeout or core cleanup code was changed.
- Strict workspace/all-target Clippy (`-D warnings`) and format checks PASS.
- Reference suite: **411 passed, 4 expected skips**; all JS/QML contracts PASS.
  Python compile, shell syntax, manifest, plugin validation and diff check PASS.
- No-Python executable conformance: **55 passed** across five actual compiled
  CLI suites, including ten setup cases; Python and host service/privilege entry
  points masked in isolated namespaces. Not installed application acceptance.
- Debug candidate SHA256:
  `c8a5b4883aeed2672c3f3bfa55d055c3cec7f7acf439b09c2da1a368c6e7d639`.
- Installed executable remains source `9de33cd`, frontend `3eb2d73`.
  Read-only final observation: Routing, disconnected, manual recovery false,
  core/auxiliary/TUN counts `0/0/0`. No private data or new prompts were involved.
