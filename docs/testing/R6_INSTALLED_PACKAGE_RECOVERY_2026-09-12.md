# R6 installed Arch package recovery gate

Status: **procedure/tooling, NOT RUN until an attended execution is recorded**.
This document must not be cited as a successful package rollback/removal result.

`tests/installed_native_package.py` is a developer acceptance tool. Python runs
the observer, not OmaVLESS; this is a real installed-package recovery gate,
separate from the Python-inaccessible application matrix. It changes no product
runtime, packaging, privilege policy or ownership/recovery contract.

## Scope and entry conditions

Use an actual Arch/Omarchy user session, with the plugin idle/closed and no other
actor changing state. This procedure tests the existing user's installed native
package, **not a fresh OS installation or Nix generation rollback**. The owner
must explicitly allow temporary package removal and be present for all normal
sudo/pacman and service authorization. The script must run as the ordinary user
in a real visible terminal, never as root or an agent-fed pseudo-terminal.

Required baseline:

- installed and running current native binary match the current package archive;
- native service enabled, startup configured Off, genuinely observed disconnected
  and `manualRecoveryRequired=false`, no owned/visible core, auxiliary core or TUN;
- no other user's visible native runtime or any visible Mihomo core;
- exact normal HOME/XDG paths, strict private directory/file modes, no symlinks;
- two retained, already reviewed local `omavless` package archives for this host
  architecture: the exact current recovery candidate and an older compatible
  native candidate. No download, build or arbitrary installation hook is allowed.

The checked-in tool accepts `--current-package` and `--rollback-package` absolute
paths plus explicit `--run`. Verify both archives are kept outside ephemeral
`/tmp` and have adequate free space. They must remain accessible throughout.
The archives are not deleted by the tool. Their complete package digests,
exact source heads and executable digests belong in the eventual sanitized
evidence record. Do not print the user's private store to inspect it.

The archive validator checks the fixed reviewed payload, excludes extra files,
install scripts, special/link/set-id members and unsafe ownership/modes, validates
package name/architecture/version/build identity, hashes the actual packaged
executable and rechecks archive identity before every transaction. `pacman`
retains normal dependency/conflict checks and confirmation. There is no
`--nodeps`, `--overwrite`, `--nosave`, recursive removal, dependency removal or
`--noconfirm` escape hatch.

## Fixed sequence

1. Record metadata and hashes/modes of private profiles, durable desired intent,
   ownership marker, bridge selector, optional generated config and route template.
   Hashes stay in memory; ordinary output contains booleans, not private content.
2. Stop only `omavless-runtime.service` after a separate human `ready` barrier;
   require human `settled` afterward and prove the runtime/core/TUN absent.
3. Normally install the older archive using `sudo pacman -U`, with its own
   pre/post human barrier. Reload the user's systemd catalog and start the fixed
   service with another barrier. Check the **running `/proc/.../exe` digest**,
   installed package identity, native owner, startup Off and clean disconnected
   facts. Private state bytes/owner/modes must remain unchanged.
4. Stop, normally reinstall the exact current archive, reload and start; repeat
   all private-state, actual executable and empty-host checks.
5. Stop and normally remove **only `omavless`** using `sudo pacman -R` with its
   own barrier. Prove installed executable and both packaged unit files absent,
   no runtime/core/TUN, and unchanged private configuration/state. The user's
   unit enablement link may remain dangling until the package is restored.
6. Reinstall the retained exact current archive and start the fixed service.
   Prove original native enablement, startup Off, clean disconnected state,
   exact actual executable and unchanged private bytes/permissions.

The tool issues no Connect, Disconnect, mode mutation, private-store write,
marker/receipt deletion, plugin disable/remove or new cutover. Stopping a
genuinely disconnected native service is not evidence of connected tunnel
shutdown or DNS restoration. Service start deliberately uses the packaged normal
login-preparation dependency; receipts are never edited to manufacture success.

## Failure/recovery boundary

The [human authorization barrier](HOST_AUTHORIZATION_ACCEPTANCE.md) applies to
each authorizing action. Human waiting has no short artificial timeout. Never
type passwords into the acknowledgement prompt. A stopped/unsettled barrier
permanently blocks that invocation: no automatic reinstall/start/cleanup follows.

On any refused assertion, package error or interrupt, the tool stops and reports
only a fixed public classification plus the need for attended recovery. There
is deliberately no `finally` which blindly starts a service or installs another
package while earlier authorization may still be pending.

An attended operator must first inspect the actual package/service/host state,
settle every dialog, and verify the preserved current archive. If package-only
recovery is needed, a **new explicit** ordinary `sudo pacman -U` of that exact
archive, followed by normal user daemon-reload/start, is the supported route.
If private bytes changed, ownership became ambiguous or manual recovery is
required, do not overwrite the saved hashes/markers, force activation or claim
rollback success. Preserve evidence and follow the owning recovery contract.

## Evidence to fill only after execution

Record exact tooling head, environment/architecture, both archive SHA-256s,
source and executable digests; each stage's PASS/FAIL; actual daemon identity;
private byte/mode preservation; dependency-checked removal and retained state;
restored package/enablement; final desired/actual disconnected state and
service/core/TUN counts. Record any failure and the exact separately attended
recovery rather than replacing a failed run with an unconditional PASS.

Deterministic policy tests exercise archive allowlists, unsafe file/metadata
refusal, strict disconnected facts, auth stops, absence of implicit recovery and
the exact dependency-preserving pacman argv. They never execute systemd, sudo,
pacman, a real package operation or a VPN transition. Passing them is not host
acceptance.
