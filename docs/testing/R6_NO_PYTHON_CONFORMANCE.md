# R6 no-Python conformance foundation

2026-09-11, local-only Try Omarchy ARM64. This is executable test evidence,
**not** installed GUI/network/startup acceptance or R6 completion.

## Two complementary layers

Subsequent fresh-setup checkpoint: the harness now includes the fixed fifth
`fresh_setup_cli` suite. All54 CLI tests pass on source `58b9856`, including nine
fresh-config cases. See [fresh preparation](R6_FRESH_CONFIG_PREPARATION.md) for
the new binary identity. The four-suite/45-test identity below is historical;
include `--test fresh_setup_cli` when regenerating the current artifact manifest.

`tests/test_native_launcher_no_python.py` executes the real `backend.sh` with a
synthetic fixed native executable and a PATH containing no interpreter or host
commands. One matrix covers 51 aliases and 14 private-stdin flows, exact argv,
arity, errors, ownership refusal and missing-native-binary safety. All eleven
tests pass. The Python test driver is outside the tested child boundary.

`tools/run-native-no-python.js` runs already compiled native test executables
under unprivileged bubblewrap. It accepts only the five named Cargo test
artifacts bound to this checkout's runtime manifest and test source paths:

The table below is historical four-suite evidence, not the current suite count.

| Suite | Actual executed tests |
| --- | ---: |
| `cli` | 22 |
| `plugin_action_cli` | 10 |
| `desktop_cli` | 6 |
| `plugin_target` | 7 |
| Total | 45 |

These test the real Rust executable against synthetic fixtures/Unix peers and
controlled desktop helpers, including a read-only daemon scenario. The CLI
boundaries execute; real provider, systemd, TUN and GUI operations do not.
The tested binary SHA-256 is
`1713849692563ae0ecaa67dc1e8e27ea40d9c054e38c5c1f573e44095fbd2b1d`, built
from runtime source `9de33cd0243261e6ce20d352135ea0b77919f56e`. Current edits
to tools/QML/docs do not change that binary; regenerate artifacts if Rust changes.

## Isolation and evidence limits

- Separate PID, network, mount and other namespaces; fresh `/proc`, `/run`,
  `/tmp` and minimal `/dev`; no host bus, GUI environment or `/dev/net/tun`.
- No private home mounted. Only source checkout, selected executables and
  system `/usr` are read-only; synthetic writable data stays in namespace tmpfs.
- All normal `/usr/bin/python`, `python3` and versioned Python interpreter
  targets present on this host are resolved and masked with non-executable
  bindings. Both PATH and absolute interpreter execution are negatively tested.
  This is not a claim about arbitrary copied interpreters or alternative PyPy
  installations on a different host.
- Normal privilege/service/network-management entry points are masked too.
  No installation, user-service change, host networking or OS-policy relaxation.
- The payload runner may use an already installed Node executable from outside
  `/usr`; only that binary is bound into the namespace, not its home directory.
- Raw test stdout/stderr are not published. Output contains only suite names,
  counts, booleans, binary digest and `installedAcceptance:false`. Test failure
  stops the sequence; missing isolation is a refusal, never an unsandboxed retry.
- Artifact metadata selects the expected source/package and unique executable
  set, but is not a cryptographic build attestation. Build from the exact clean
  candidate with locked Cargo immediately before recording final evidence.

Ten deterministic Node tests validate artifact selection/bounds and the fixed
namespace/masking plan without running commands. A separate launcher suite
tests command mappings; neither one substitutes for the 45 executable tests.

## Reproduction

Use one Cargo writer and the existing shared target in the VM. First generate a
fresh bounded JSONL artifact manifest with `cargo test --locked -p
omavless-runtime --test cli --test plugin_action_cli --test desktop_cli --test
plugin_target --test fresh_setup_cli --no-run --message-format=json`. Redirect
its output outside Git. Keep the build profile/environment consistent with the
preceding build to avoid an unnecessary second Cargo build in a small VM.
Then run:

```sh
node tools/run-native-no-python.js --artifacts /absolute/cargo-artifacts.jsonl
```

To run those same tests against the installed executable instead of the Cargo
executable:

```sh
node tools/run-native-no-python.js --artifacts /absolute/cargo-artifacts.jsonl --installed
```

This fixed option selects only `/usr/bin/omavless`; there is no arbitrary
program/path option. A read-only bind overlays the test harness's embedded
`CARGO_BIN_EXE` path **inside the mount namespace only**. The host package and
Cargo files remain unchanged. The tool hashes the selected executable before
and after all suites, reports `binarySource: installed` or `cargo`, and still
reports `installedAcceptance:false`. Fixtures, peer replies and desktop helpers
remain synthetic even when the executable comes from an installed package.
It does not restart or test the already running host daemon, or prove graphical,
login, package-manager, DNS or live-tunnel acceptance. A package incompatible
with the current conformance corpus fails; it is never silently replaced with
the Cargo binary or tested unsandboxed.

No `sudo`, installed package replacement, host daemon restart or current private
fixtures are needed. Never point the tool at unreviewed source/artifacts and
claim the resulting counts as an installed application gate.

## Still required

The [dependency audit](R6_PYTHON_DEPENDENCY_AUDIT.md) records fresh initialization,
installer/helper policy, remaining IPC/reporting and package/login boundaries.
Complete installed Python-unavailable acceptance still needs the normal QML,
profile/subscription, routing, connect/disconnect, diagnostics, login and
upgrade/rollback paths with real host identity and controlled authorization.
System Python must not be removed to simulate that test. DNS/provider and V0
fixture gaps remain separate and unchanged.
