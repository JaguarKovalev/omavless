# Native 0.8.0 final candidate — preparation, not publication

Environment: Try Omarchy ARM64 on Apple Silicon, 2026-09-14.
Starting main: `a5dde682533a77ed52e0d477f3f6a71472d59999`, green final CI.
The owner approved final-source/package preparation and local acceptance;
the later x86_64 Omarchy pass and marketplace publication remain separate.

## Scope and preservation

- Workspace, its seven local lockfile package versions and frontend manifest
  change coherently from `0.8.0-rc.1` to `0.8.0`. No dependency update.
- No production Rust function, QML layout, host policy, unit, private-store
  schema or ownership transition changes. The compiled version affects
  diagnostics and the subscription User-Agent, so the real final ELF must be
  rebuilt and its installed response checked, not relabelled from RC.
- The QML frontend is shared between ARM64 and x86_64; only native packages are
  architecture-specific. Version/source must match across each delivered pair.
- RC tests now create explicit RC inputs rather than depending on the moving
  root version. Stable and RC archive/installer/refusal coverage are both kept.
- Existing scoped R6, clean onboarding and UI evidence is retained for unchanged
  behavior. AUTO-1, DNS/provider findings, V0 and #135 are not declared passed.

## Acceptance ledger

Actual build hashes, exact source and executed gates are appended below when
available. Pending entries must not be inferred from a version number or a
successful synthetic archive test.

| Gate | State at initial source checkpoint |
| --- | --- |
| Source/version coherence and local static suite | 235 tests: 233 PASS / 2 existing SKIP; JS/QML, plugin validation, manifest, shell syntax and diff PASS |
| Full Rust checks and locked optimized ARM64 build | Pending |
| Final ARM64 package/frontend assembly and inspection | Pending |
| Actual attended RC-to-final update | Pending; normal authorization required |
| Actual final-version support response and frontend identity | Pending |
| Short final-binary connect/disconnect and cleanup | Pending; separate guarded effects |
| x86_64 final source/package/host pass | Not run on this ARM64 VM |
| Version tag, release upload, marketplace revision | Not performed; separate approval |

Private recovery archives/backups and any real fixture/capture stay outside Git.
Use the existing [per-effect authorization guard](HOST_AUTHORIZATION_ACCEPTANCE.md)
and [upgrade-only package gate](../../packaging/release/README.md). Never auto-type
ready/settled, force a password deadline, retry an unsettled effect or alter
ownership/private state to make acceptance pass. On a failure, report the
sanitized stage and inspect with the owner before another host effect.

Final PC steps are in the [release handoff](NATIVE_080_RELEASE_HANDOFF_2026-09-14.md).
Do not publish or advertise an architecture whose exact artifact was not tested.
