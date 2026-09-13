# Current delivery status

Updated 2026-09-13. This is the compact current-state entry point; the detailed
[delivery roadmap](../../DEVELOPMENT_ROADMAP.md) preserves the implementation
history. GitHub's actual main/PR state is authoritative for publication.

## Accepted native checkpoint

| Track | Current state |
| --- | --- |
| R0–R4 | Rust protocol, profile/domain/store and core foundations accepted; do not restart their migration |
| R5 / T1 | Rust owns the activated native application's state, mutations, background work and Mihomo lifecycle; restored QML is a client of that owner |
| R6 | Native installed path closed under the owner-approved scope, with deliberate installed Python-absence evidence |
| Enabled login autoconnect | AUTO-1 open; Last/pinned fresh-login acceptance is deferred, not PASS; default is Off |
| Network/DNS | Separate unresolved investigation; failed probes remain failures and are not hidden by migration closure |
| Native distribution | Reviewed local Arch package and explicit native-only frontend install accepted; no automatic marketplace migration or release implied |
| V0 / #30 | Draft; accepted exact XHTTP evidence preserved; missing-family/UDP-restricted evidence remains unavailable |
| T2 | Migration prerequisite satisfied for the accepted native path; client implementation is a separate task, not part of this integration |
| P4, K1, NixOS and other host tracks | Own design/fixture/security/host gates still apply; not promoted by R6 |

The [R6 closure](../testing/R6_LOCAL_CLOSURE_2026-09-13.md) states the exact scope,
tested runtime/frontend/package identities and exceptions. The
[integration evidence](../testing/R6_PUBLICATION_CANDIDATE_2026-09-13.md) records
the final local suites and the mapping of intermediate Drafts #214–#237 to
this tree. It is a dated preparation report, not a permanent publication block.
Do not merge those obsolete Draft implementations over the integrated result.

## What users install

The immutable published marketplace 0.7.0 snapshot remains
`69fe05b03129a23664fff3f8289821a7b7f80095`.
Neither a main merge nor the presence of Rust sources installs a native binary,
runs Cargo, grants capabilities, changes an ownership marker or enables VPN
startup on a user's machine.

The [native guide](../user/NATIVE_INSTALL.md) is the supported reviewed-candidate
path: install the package, explicitly activate once, then install its matching
frontend with `./install.sh --native-only`. An activated native owner never
falls back to Python. The ordinary installer still provides the distinct
legacy-compatible payload; changing that default and publishing native release
artifacts needs a separate distribution checkpoint. Retained Python code is not
permission to grow a second native lifecycle owner.

## Next work, in order

1. Publish/verify this integration through its exact-head PR checks; close the
   included or demonstrably superseded Drafts only after their integration is
   present on main. Once that is done, do not repeat it from this checklist.
2. Prepare clear native release/package installation and upgrade communication,
   plugin-page screenshots and release metadata without silently changing the
   default install route or the published 0.7.0 identity.
3. Address [AUTO-1](LOGIN_AUTOCONNECT_FOLLOWUP.md) and the
   [network investigation](../testing/R6_NETWORK_DIAGNOSIS_2026-09-13.md) with
   their actual reproducible host scenarios; no repeated password/dialog loops.
4. Scope a small T2 client checkpoint or another explicitly selected roadmap
   task on the existing Rust owner. Preserve accepted UI unless the task
   deliberately changes it under the [UI/UX contract](UI_UX_CONTRACT.md).

Historical acceptance reports retain their original heads and outcomes. Their
old "Python still owns production", "R5 incomplete" or "publication withheld"
sentences describe those earlier checkpoints, not the current native state.
Do not erase negative evidence, advertise untested host behavior or convert
deferred work to PASS while synchronizing documentation.
