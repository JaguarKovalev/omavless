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

## Built exact-source checkpoint

Artifact source: `b7fd0a99b8b169f0933e5f43ea4389642015193a` (PR #248).
Later evidence-only commits do not relabel this source identity.

- Full local Rust checks: **957 PASS, 10 ignored**; formatting, all-target
  Clippy and language-neutral R0 parity PASS. Rust 1.98.0, ARM64.
- Locked, offline optimized `omavless` build PASS.
- GitHub CI for that source PASS:
  [run 34851807801](https://github.com/k-kostin/omavless/actions/runs/34851807801).
- Explicit `--stable` offline assembly PASS. All three `SHA256SUMS` entries
  verify; the extracted common frontend passes `omarchy plugin validate`.
- The strict installed-package inspector accepts the real archive: expected
  members, modes, schema-3 identity, version, architecture, source, ELF hash
  and user units. No package hooks, symlinks or private fixtures are included.
- Pre-update read-only checks identify the retained RC archive as exactly the
  installed/running RC executable and units. Private state is backed up outside
  Git; startup is Off and the baseline is observed disconnected without manual
  recovery, Mihomo or TUN. This is preflight, not update acceptance.

| Artifact | SHA-256 |
| --- | --- |
| Optimized ARM64 ELF | `12afa0a6ae279d23f1b89426d47fdd478e0ef1987f171a92924b9c925f6f0458` |
| `omavless-0.8.0-1-aarch64.pkg.tar.zst` | `454662a76f106af5b2f4ee8ab3ef4626a1641981fd9e6baa0a2ca8d91dc7983d` |
| `omavless-0.8.0-frontend.tar.xz` | `17a8ab16c950500b2781b670b6640f8bbcc28dc5b1ad42536bfc657a23c28320` |

These artifacts are retained locally, **unpublished**. Their manifest records
`caller-supplied-prebuilt` provenance; inspection plus the recorded actual local
build establishes this checkpoint, not a claim of a release-service build.

## Actual RC-to-final update and frontend identity

The attended package invocation reached the installed final package and started
its service, but recorded `human_authorization_unsettled` rather than a complete
gate PASS. The owner confirmed a typo at a `settled` prompt after entering the
normal sudo password. The runner's generic `stage: stopped` does **not** identify
which effect completed; no successful final acknowledgement is fabricated.
No reinstall, automated acknowledgement or compensating retry followed.

Separate read-only post-update verification proves:

- pacman reports `omavless 0.8.0-1`;
- `/usr/bin/omavless` and the running service ELF match the final hash above;
- both installed user units match the inspected final package;
- native ownership, enabled service and startup-Off configuration remain;
- all six private store/state/config fingerprints, modes and owners match the
  retained pre-update backup;
- observed disconnected, no manual recovery, no Mihomo/TUN/controller leftovers.

The extracted final frontend was then installed through its normal installer:
24 runtime-relevant files match byte-for-byte, manifest version is `0.8.0`,
and the plugin remains enabled. No tunnel was started by installation. The
actual final `diagnostics export` response passes the installed QML support
parser as schema 3 / Rust `0.8.0` (bounded 2,155-byte safe projection), with
onboarding completion preserved. This rechecks the changed version boundary;
the unchanged visual Copy report acceptance is retained from #245.

Thus installed update **outcome and data preservation are verified**, while
the original guarded invocation's final acknowledgement remains incomplete.
A separate attended final-binary network gate is recorded below when finished;
package installation is not repeated to manufacture a cleaner transcript.

## Final-binary live gate — PASS

The separate checked-in `installed_native_acceptance.py` gate completed with
exit 0 and a durable PASS result. Its own ready/settled barriers completed;
this is not a retry of the interrupted package invocation. One existing usable
VLESS fixture was used without printing its record, name or credentials.
Mihomo: `v1.19.30`, Linux ARM64, Go `1.26.6`, `with_gvisor`.

| Check | Actual result |
| --- | --- |
| Connect / actual controller mode | PASS, Full VPN; semantic connect 153 ms, excluding human waits |
| Ownership / resources | PASS, one native service owner, one descendant Mihomo and one TUN |
| Private Unix controller | PASS, same-user peer and expected active mode |
| TCP controller | Absent; generated controller policy and read-only PID-attributed socket inventory PASS; only expected loopback proxy/TUN forwarder listeners |
| Bounded HTTPS probe | PASS, generic `https://example.com/`, explicitly bound to the observed TUN; RX/TX counters increased |
| Disconnect / cleanup | PASS, fresh observed disconnected, no recovery/core/TUN/controller leftovers |
| Restoration | PASS, Routing restored; original last-profile fixture retained |
| Privacy | PASS, bounded fixed-vocabulary result; private inputs/results remain outside Git |

Independent final reads still identify the same final ELF and report 37 profiles,
one subscription, startup Off, observed disconnected Routing and
`manualRecoveryRequired: false`. Native service remains active/enabled; old
service inactive; Mihomo/TUN/auxiliary counts are 0/0/0. Plugin enabled.
No protocol-family claim beyond this representative VLESS case is made.

Applicable ARM64 final-candidate preparation is complete, with the package
acknowledgement qualification above preserved. The exact x86_64 package/build
and final PC host checks remain required for the planned release. No version
tag, uploaded release asset or marketplace change was made.
