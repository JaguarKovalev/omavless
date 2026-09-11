# Fresh native config preparation

Local source candidate `58b98569f15dfbad2371af1bf042ababaac387a4`,
2026-09-11, tested in Try Omarchy ARM64 with synthetic private roots. This is
**preparation, not ownership activation, complete fresh installation or R6**.
The installed native executable/plugin was not replaced in this checkpoint.

## Fixed executable boundary

`omavless setup initialize` accepts no arguments, path, payload, force flag,
provider input or IPC equivalent. It creates only the canonical initial
`$HOME/.config/omavless/profiles.json` and `route-template.yaml`. Missing `.config`
and the app directory are created mode0700; existing ordinary same-user `.config`
may be0755 but the app directory must already be0700. Files are mode0600.
No existing directory is chmodded to force admission.

The exact store bytes reuse `store_bootstrap::EMPTY_STORE_PAYLOAD`, whose test
compares them against actual Python `empty_store()`. Profiles/subscriptions,
active/last pointers and custom rules are empty; startup is explicitly configured
Off and onboarding remains incomplete. The template is the unchanged checked-in
default, like the legacy default policy. It is copied, not validated by launching
Mihomo, fetched from the network or installed as active config. Onboarding can
select another bundled policy later. No new protocol semantics are introduced.

The bounded response contains only:

```json
{"schemaVersion":1,"outcome":"prepared","createdFiles":2,"ownershipActivated":false,"startupEnabled":false,"onboardingComplete":false}
```

`createdFiles` is0 on an exact no-op retry or1 when finishing an unchanged
partial pair. Success is not runtime/core/network readiness. Failures use a fixed
bounded English vocabulary without paths, existing file contents or raw errors.

## Admission and failure policy

- Absolute UTF-8 roots, maximum4096 bytes, no controls/parent traversal; reject
  symlink/non-directory ancestors. Home and config parents must be same-user and
  not group/other-writable. Runtime base must already be a private same-user
  directory. `OMAVLESS_HOME` overrides refuse.
- Canonical state paths use existing HOME/XDG_STATE_HOME/XDG_RUNTIME_DIR rules.
  This command does not change the established HOME-based config location or
  introduce alternative XDG config-store semantics.
- Native runtime directory must be absent. An existing canonical app-state
  directory must be private and empty: markers, bridge targets, desired state,
  login receipts, recovery barriers and unknown members all refuse untouched.
- Existing config may contain only the two initial files, each absent or
  byte-identical to the private initial payload. Valid but user-modified data,
  malformed files, wrong permissions, symlinks, FIFO, extra active-config files
  and unknown entries refuse. This command is not legacy migration or repair.
- Reuse the shared migration lease. Validate member contents only while holding
  it, so a second initializer cannot mistake the first one's create-exclusive
  temporary file for an unrelated installation. Busy refuses before config
  creation. The reused locked bootstrap verifies its lease's path/UID authority.
- Template and store use existing create-only atomic primitives; neither target
  is replaced. Recheck exact contents and unused ownership state before success.
  An uncooperative racing file is never overwritten to match defaults.
- Files are individually durable, **not one multi-file atomic transaction**.
  Interruption may leave newly created private directories and one initial file.
  Exact unchanged partial pairs can be retried; ambiguous/modified members or
  unknown scratch artifacts refuse. There is no rollback deletion or automatic
  repair of user data. Operational migration-lock creation is permitted.

No service process, shell/privileged helper, daemon/socket client, ownership
publication, desired-state write, autoconnect, route or DNS operation occurs.
Current native runtime and legacy behavior are not cut over by this command.

## Acceptance

- Nine real-executable integration tests cover fresh defaults/permissions,
  exact no-op retry, partial-pair completion, changed/invalid private files,
  state/receipt/runtime refusal, symlink/mode safety, FIFO/oversize refusal,
  lock contention, concurrent creators and invalid CLI/environment input.
- One additional bootstrap regression rejects a lease for another runtime path.
  The existing actual-Python empty-store parity remains passing.
- Full locked Rust workspace: **939 passed, 10 existing ignored**. Strict
  workspace/all-targets Clippy and format checks PASS.
- Full reference/frontend suite: **411 passed, 4 expected skips**, all JS/QML
  contracts; compile, shell syntax, manifest, plugin validation and diff PASS.
- The no-Python harness now admits the fixed `fresh_setup_cli` artifact as its
  fifth suite. **54 actual compiled CLI tests PASS**, including all nine new
  setup cases, with Python executables and privilege/service entry points
  masked, no host home/bus/TUN/GUI and isolated synthetic writable state.
  Candidate executable SHA256:
  `680903581544f09ad6d0fa5f5a1685f4b41ef93ac1931268b22117319192657f`.
  This is executable conformance, not an installed fresh-user host gate.

## Next owning slice

The existing packaged `cutover activate` still assumes an initialized legacy
installation, including disabled legacy-unit installation facts and existing
state-parent setup. A clean host with no Python unit requires an explicit,
strict absent-legacy admission/composition checkpoint plus deterministic host
tests, followed by actual packaged acceptance. Do not fake a disabled legacy
unit, write ownership/receipt files by hand, or claim this preparation alone
unlocks `install.sh --native-only` (which requires committed Rust ownership).

Then test the full fresh-user onboarding/import/startup-Off path, activation
failure/retry/recovery and actual login. Package upgrade/removal/rollback,
support-report composition, controlled DNS/auth and installed full Python-absence
remain separate R6 gates. V0/private fixtures are unchanged and unrelated.

All work is local-only, with no push/main changes or package install. The
installed native-only frontend remains `3eb2d73`, runtime build `9de33cd`.
Only the regenerable Cargo incremental cache was removed for disk space;
source, packages, private fixtures and installed binaries were preserved.
