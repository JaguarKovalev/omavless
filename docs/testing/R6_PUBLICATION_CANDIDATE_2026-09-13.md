# Native R6 integration — local publication candidate

Historical preparation status: **local checks green; publication withheld at
the time of this report**. The owner subsequently authorized publication on
2026-09-13. Use [current delivery status](../roadmap/CURRENT_STATUS.md) and the
actual integration PR state for delivery; the snapshot below preserves the
pre-publication evidence and cleanup plan.
This is the final preparation checkpoint, not a merge, tag or marketplace
release. The [scoped R6 closure](R6_LOCAL_CLOSURE_2026-09-13.md) remains the
acceptance authority. No VPN transition or authorization request was made in
this preparation session.

## Source and installed identities

- Branch: `codex/local-native-probe-execution`.
- Fetched main: `27e2e793f0e14f19f41dce947e06667ca9bf5ec3`.
- Main is an ancestor: initial divergence was **0 remote-only / 135 local-only**
  commits at `95b68d6be1ad77563eef2bca2eb6a80b9decba6f`.
- Final test-code checkpoint: `4de1a7c` (isolated QML gate); subsequent changes
  in this session are documentation only. Obtain the final documentation head
  with `git rev-parse HEAD`, rather than treating this embedded SHA as HEAD.
- Rust/package implementation remains
  `7b75b747883d66a05f1f2b321d42f194c6040b5c`; no later `crates/` or `packaging/`
  production diff. Installed package: `0.0.0.r492.g7b75b747883d-1`, aarch64.
- Frontend implementation remains
  `13717a75264aa2be350bdf741d57b4f1210bcbad`; no later runtime frontend/launcher
  diff. Do not reinstall or reconnect merely for documentation/test additions.
- Installed and running executable SHA-256:
  `fe04fba32d4e135d56e929350cd0296d3f89168e06d6ebba9a6de089aec4b786`.

The accumulated implementation is substantial: at the start of preparation,
218 files, 30,853 additions and 438 deletions relative to main. It is the
integration of separately developed/accepted R5/R6 slices, not a new single-step
rewrite. Keep the component commits and their recorded acceptance identities.

## Final local checks

Try Omarchy ARM64, 2026-09-13. Full suites ran at `95b68d6`; the only subsequent
test-code addition is the separately exercised compile gate at `4de1a7c`.

| Check | Result |
| --- | --- |
| Rust workspace | 957 passed, 0 failed, 10 ignored |
| Rust fmt / all-target Clippy with warnings denied | PASS |
| R0 parity runner | 2 cases matched, 0 mismatches |
| Full reference/launcher Python suite | 460 run: 456 passed, 4 skipped |
| Same full suite with installed Mihomo opt-in | 460 run: 459 passed, 1 skipped |
| JS presentation/bridge suites and QML contracts | All PASS; main-panel 37, startup 7, support 16, file-export 12 |
| Actual QML compilation | Candidate Panel PASS; valid fixture PASS; invalid Process/Timer fixture rejected with exit 1 |
| Python compilation / shell syntax / manifest JSON | PASS |
| Omarchy plugin validate | PASS |
| Worktree and entire main-to-candidate diff check | PASS |
| Local Markdown file links | 299 resolved, 0 broken |
| qmllint | NOT RUN — not installed; actual component compilation recorded separately |

Rust invocation used `CARGO_INCREMENTAL=0`, dev/test debug info disabled,
`CARGO_BUILD_JOBS=2`, `RUST_TEST_THREADS=2` to fit the VM. No test retries or
deadline changes were needed in this pass. The earlier timing-sensitive test
failure remains in the historical ledger, not declared root-caused by this pass.
The remaining Python skip is the root-only refusal execution; portable guard
coverage passes. Installed Mihomo is Meta v1.19.30, linux arm64, Go 1.26.6,
`with_gvisor`. Synthetic core configuration checks do not prove provider or
live-tunnel interoperability. No new GitHub Actions run was requested.

## Publication-session CI correction

Integration PR #238's initial run at `da941b0` failed before the owned-helper
resource test reached readiness:
`core::tests::helper_resources_are_drained_even_after_leader_exit_or_term_spawn`
returned `SpawnFailed` at its freshly written executable fixture. See
[the original failed run](https://github.com/k-kostin/omavless/actions/runs/34746388452).
The fixed public error did not retain the OS errno; ETXTBSY/publication timing
is a plausible explanation, not a proven errno diagnosis.

The correction executes the same checked-in helper as an executable instead of
rewriting/chmodding a temporary executable during concurrent process spawning.
The Rust change is exclusively inside `#[cfg(test)]`; the production supervisor
prefix is byte-identical. The helper remains test-only and is not installed by
the native package. No spawn retry, production deadline, cleanup assertion,
private fixture or installed VPN behavior changed.

The focused helper test passes. The complete local two-thread gate again passes
957 Rust tests / 0 failures / 10 ignored, fmt, Clippy and R0 parity. The full
Python suite again runs 460 tests / 4 skips, with all JS/QML checks passing.

An additional four-thread VM run did **not** pass: the helper test succeeded,
but `auxiliary_core::tests::revoke_during_chunk_cleanup_never_reopens_stale_lease`
returned `Cleanup` while finishing the successor lease (runtime library: 637
passed / 1 failed / 6 ignored). The same unchanged auxiliary test passes alone
and in the standard two-thread gate; a later VM observation showed load average
17.50 on eight vCPUs. This is unresolved load-sensitive evidence, not proof of
its precise internal cause. Preserve it alongside earlier auxiliary timing
observations; no cleanup boundary was relaxed to obtain a pass. Cloud acceptance
must still pass on the corrected exact head before merging.

## Remote Draft reconciliation

Open PR metadata and remote refs were inspected, not modified. A later repeated
GitHub metadata query returned EOF; this does not invalidate the successful
initial fetch/list, but publication must refresh again. Main has not been
rebased or force-updated. All PRs below were Draft when inspected.

| Draft PR(s) | Local integration evidence / handling after integration merge |
| --- | --- |
| #214–#217, #219–#220 | Exact heads already ancestors; close as included after the integration merge |
| #218 | `08b3a60`: private controller permissions; range-diff differs only in neighboring module context |
| #221 | `966f70b` plus `ff12fcc` watchdog and `b3537b5` visible provider progress fixes |
| #222 | `1eaa73b` plus `ff12fcc`; historical installed report restored during this preparation |
| #223 | `8ab346d`, `dd55e69`; later startup implementation supersedes read-only placeholder |
| #224 | Previously absent compile gate reconciled as `4de1a7c`; corrected its stale five-second documentation to the actual 30-second bound |
| #225 | `41f970b`; later confirmed Save As/private export fixes retained |
| #226 | Exact patch-equivalent work, including `a707421`; subsequent validated target lookup retained |
| #227 | `13f03a7`, `72ab22a`; range-diff shows combined frontend/test context adaptations |
| #228 | `05f0d34`, `0538805`, `771e8ad`; later owner-hidden main Test section remains hidden |
| #229 | `5b9b27c`, `a2c5ce1`; onboarding capability/revision fences preserved |
| #230 | `44bf548`, `e3fb10e`, `e78247d`; current Details stays explicit and private |
| #231 | `5a6d13c`, `cdd6b27`; later support schema and Save As changes retained |
| #232 | Patch-equivalent `f7b11a2`; later native login integration retained |
| #233 | Patch-equivalent `9cea15c`; later probe execution/ownership integration retained |
| #234 | `1b0ba10`, `c465a0d`, `aeff03a`; range-diff preserves semantics across module/UI context |
| #235 | Patch-equivalent `fba22a1` status-cache test clock |
| #236 | `4f14010`, `ddaff1d`, `51ee820`; missing historical installed ping observations restored, including failures; main latency section stays hidden |
| #237 | Patch-equivalent `9e2ce6b` stopped-runtime error |

Proof uses ancestry, `git cherry` patch equivalence, paired range-diffs and
owning-file inspection. Rebased/composed patches are not claimed to retain
identical hashes. Added-file inventory across 29 related fetched branches found
no remaining absent files after reconciliation. Current full tests gate the
combined result; old Draft heads must **not** be blindly merged afterward.

Keep #30 and #135 out of this cleanup: #30 retains its exact accepted XHTTP
evidence and unavailable-family Draft status; the separate authorization/mode
issue is not silently closed by local migration acceptance. No PR or branch was
deleted in this session.

## Repository hygiene and evidence

- Source remains in `plugin/`, `crates/`, `packaging/`, `tests/`; documentation
  has [an audience index](../README.md) and [current evidence index](README.md).
- Historical reports remain at stable paths, with current-status pointers
  rather than erased failures or repeated obsolete task lists.
- Tracked artifact inventory found only the intentional existing `preview.png`;
  no build archives, log dumps, private captures or generated result files.
- Tracked-text scans found no private-key block, GitHub-token signature or
  private fixture-directory reference. Available private snapshot identifiers
  were compared internally against tracked text with zero matches; no values
  were printed. This is a scoped audit, not a proof that arbitrary secrets can
  never exist. Synthetic credential-shaped regression inputs remain intentional.
- Root launchers and manifest stay at their supported installer locations.
  No cosmetic directory move that could break installation was attempted.
- The local test/build cache is ignored, not part of the publication diff.
  No private input/result file is included in the commits.

## Final read-only installed check

All 20 tracked frontend/launcher runtime files match the installed native-only
payload. Installed legacy backend/remover are absent; disk and running native
binary both match the SHA-256 above. The original native runtime process is
unchanged. Plugin enabled; saved startup Off / last / Routing.

Fresh observation reports connected Routing, desired profile matching the owned
core, one visible Mihomo and TUN, zero auxiliary Mihomo and no manual recovery
flag. The daemon has one Mihomo child. The owned controller configuration is
verified and `ss` reports no attributable TCP listener for that child. This
read-only pass does not add live DNS/routes/HTTPS or independent service/TUN
ownership acceptance; those observation verification fields remain false.
No shell/plugin reload, core restart, connection change or password dialog was
triggered. Prior attended gates retain their own exact scope.

## Preserved limitations, not new R6 tasks

1. [AUTO-1](../roadmap/LOGIN_AUTOCONNECT_FOLLOWUP.md) is OPEN; startup Off is the
   default and the current saved preference. Enabled Last/pinned acceptance was
   explicitly moved outside scoped R6 by the owner.
2. Network/DNS/provider investigation stays open. Failed masked HTTPS and ping
   observations remain failed; this session did not retry private live traffic.
3. Additional owner-deferred Save As cancel/overwrite variants remain deferred.
   Hidden Test/latency controls have an explicit restoration contract.
4. Marketplace distribution/presentation, bare-metal/x86 and NixOS acceptance,
   and V0 missing-family maturity are not implied by the local Rust retirement.

## Publication handoff — only after owner approval

1. Fetch remote refs again. If main or any owning Draft changed, reconcile the
   actual new commits; do not blindly force-push or assume this report is current.
2. Verify the candidate is clean and compare its runtime paths with the accepted
   identities above. Docs/test-only changes do not invalidate unchanged live
   runtime evidence. A new runtime diff requires its affected gates again.
3. Publish this integration branch and use one integration PR against main,
   linking this report and the scoped closure. Preserve the component history;
   do not replay the old Draft stack or directly overwrite main.
4. Inspect CI after publication. Merge only with owner authorization and green
   applicable checks. Verify actual resulting main SHA, rather than assuming a
   successful command means it merged.
5. Close superseded/included Drafts #214–#237 with the integration reference;
   delete a branch only after verifying its work and acceptance evidence are
   included. Preserve any unexpected unique commits and unrelated Drafts.
6. Separately prepare the user-facing plugin page/native distribution update.
   Main integration does not install an Arch package, activate Rust ownership
   for marketplace users, bump version 0.7.0 or publish a new marketplace SHA.

There is no planned additional native implementation or attended R6 button/
password marathon before this handoff. Real remote changes or failed future
publication checks must still be handled rather than waived.
