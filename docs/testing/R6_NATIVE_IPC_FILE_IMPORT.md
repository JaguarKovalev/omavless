# Native IPC file import — local checkpoint

Candidate: `2fd0d2ba281793133b802347866db7dc369fbef0`, Try Omarchy ARM64,
2026-09-11. Local-only; no push/merge or R6 completion claim.

`omarchy-shell kdk.omavless importConfig /absolute/local/file` now requests
native acquisition and preview. Immediate `ok: confirmation required` means
the request was admitted, **not** that the file was read or anything imported.
Later read/preview failures appear as fixed safe errors in the panel. The user
must confirm the profile or subscription through its existing native UI.

The native branch does not derive a name from the filename or silently replace
an existing record. Profile naming and duplicate refusal use the existing
confirmation path. Subscription input shares canonical URL validation and
duplicate detection; remote fetching/saving still requires confirmation.
Legacy IPC behavior is unchanged. This intentionally changes native IPC from
an unavailable operation to interactive confirmation, not headless mutation.

The fixed launcher alias `native-import-path` invokes only `omavless desktop
file-read`. The caller-selected path travels to the helper through stdin, never
new child argv, diagnostics or logs. The external shell caller necessarily
supplies its path as its IPC argument; do not put credentials in filenames.
QML admits absolute UTF-8 paths up to 4096 bytes, rejecting controls, malformed
surrogates and parent traversal. The existing Rust helper enforces filesystem,
symlink/FIFO and content bounds; QML then applies the canonical preview bounds.
Cancellation clears retained path input. Exact context, owner, instance and
revision admission is rechecked before stdin release and after completion.
No Rust lifecycle, store schema, privileged action or Python fallback is added.

## Evidence

- 13 native import JS cases execute production functions/Process-start handler,
  including 5 new cases for path bounds/inert metacharacters/private stdin,
  cancellation/staleness, subscription confirmation/duplicates and the actual
  IPC handler's fixed responses/no legacy replacement.
- 41 launcher tests pass; the exhaustive no-Python launcher matrix includes
  this fixed alias, arity, stdin preservation and failed-owner refusal.
- Full local gate: 405 Python tests, 4 expected skips; all JS/QML contracts,
  compile, shell syntax, JSON, plugin validation and diff check pass.
- Actual installed IPC: synthetic single-profile file opens profile preview;
  synthetic subscription file opens masked subscription confirmation; neither
  is saved. Missing file produces the generic source error without its path.
- Cached native `status` and `diagnostics` now respond with the new public
  projection, including disconnected state and zero core/TUN counts.
- Actual selected-profile UI export now succeeds. Its regular, same-user,
  non-symlink mode-0600 output matches the Rust semantic export byte-for-byte.
  The temporary credential-bearing export is removed after verification;
  the source profile is untouched. No private content enters this report.
- Settings scroll then panel close/reopen returns main to the top. Repeated
  after verifying the current running frontend, not only disk identity.

## Installation caveat and boundaries

Atomic install plus rescan left old QML IPC behavior active in this shell.
One supported `omarchy restart shell` loaded the new implementation; the
changed IPC responses and actual new path establish running-code evidence.
Disk byte identity alone is insufficient. This supersedes any inference that
the prior disk-identical installation necessarily activated new QML immediately.
No automatic shell restart policy is added by this checkpoint.

Input automation once lost dialog focus and sent a harmless temporary path to
the agent terminal. No credentials were involved. The successful repeat used
one bounded flow and checked the expected layer-shell dialog before typing.
Raw screenshots remain private outside Git.

Rust binary stays at recorded source `9de33cd`; frontend/launcher is the exact
candidate above. No VPN transition, sudo or polkit request was initiated.
Final inventory remains 22 profiles/1 subscription; Routing/disconnected,
no manual recovery, Mihomo/auxiliary/TUN 0/0/0, plugin enabled.

Fresh initialization, native-only distribution, complete support composition,
login/package recovery, DNS/auth and installed full Python-absence remain
separate R6 gates. This does not claim unavailable-protocol evidence.
