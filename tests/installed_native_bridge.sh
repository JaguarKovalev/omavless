#!/bin/bash
# TEST TOOL ONLY: reviewed disposable-account wrapper supplies real installed
# plugin/daemon and a Python-inaccessible user-manager mount namespace.
# Exercises file contents, NOT desktop clipboard/picker or visible UI acceptance.
set -euo pipefail
umask 077
[[ $# == 1 && $1 == --run && $EUID -ne 0 &&
   ${HOME:-} == /home/omavless-r6-domain &&
   $(id -un) == omavless-r6-domain &&
   ${XDG_RUNTIME_DIR:-} == /run/user/"$EUID" &&
   ! ${OMAVLESS_HOME+x} ]] || { printf '%s\n' 'BRIDGE GATE REFUSED: disposable account required'; exit 2; }

stage=python_absence
scratch=$(mktemp -d "$HOME/bridge-evidence.XXXXXX")
exec 2>"$scratch/errors"
feed_pid=
qml_pid=
stop_qml() {
  [[ -n $qml_pid ]] || return 0
  kill "$qml_pid" 2>/dev/null || true
  for ((j=0;j<20;j++)); do kill -0 "$qml_pid" 2>/dev/null || break; sleep 0.1; done
  kill -KILL "$qml_pid" 2>/dev/null || true
  wait "$qml_pid" 2>/dev/null || true
  qml_pid=
}
cleanup() {
  local code=$?
  stop_qml
  if [[ -n $feed_pid ]]; then kill "$feed_pid" 2>/dev/null || true; wait "$feed_pid" 2>/dev/null || true; fi
  if ((code)); then printf 'BRIDGE GATE FAIL: %s\n' "$stage"; fi
}
trap cleanup EXIT
for interpreter in /usr/bin/python /usr/bin/python3 /usr/bin/python3.*; do
  [[ -e $interpreter || -L $interpreter ]] || continue
  if "$interpreter" -c 'raise SystemExit(0)' >/dev/null 2>&1; then exit 1; fi
done
! command -v python >/dev/null 2>&1
! command -v python3 >/dev/null 2>&1
[[ -x /usr/bin/node && -x /usr/bin/quickshell ]]
plugin="$HOME/.config/omarchy/plugins/kdk.omavless"
[[ -f $plugin/plugin/Service.qml && ! -L $plugin/plugin/Service.qml && -f $plugin/backend.sh ]]
# Service destruction uses the REAL removal watcher. No fake shell registry is
# supplied: this account has no running Omarchy Shell, so registry is Unknown,
# never Disabled. Keep the installed manifest present; final daemon observation
# confirms the watcher did not stop it. This is not shell registration evidence.
[[ -f $plugin/manifest.json && ! -L $plugin/manifest.json ]]
jq -e '.id == "kdk.omavless"' "$plugin/manifest.json" >/dev/null

stage=loopback_fixture
# This local HTTP server is solely a synthetic provider fixture. Rust still
# performs its actual production fetch/validation/transaction path.
/usr/bin/node - "$scratch" >"$scratch/feed.log" 2>&1 <<'JS' &
const fs = require('fs');
const http = require('http');
const path = process.argv[2];
let requests = 0;
const profile = 'vless://00000000-0000-4000-8000-000000000011@192.0.2.11:443?security=tls';
const managed = 'vless://00000000-0000-4000-8000-000000000012@192.0.2.12:443?security=tls';
fs.writeFileSync(path + '/profile.txt', profile, {mode:0o600,flag:'wx'});
const server = http.createServer((req,res) => {
  if (req.method !== 'GET' || req.url !== '/feed' || ++requests > 8) {
    res.writeHead(400); res.end(); return;
  }
  fs.writeFileSync(path + '/request-count', String(requests), {mode:0o600});
  res.writeHead(200, {'Content-Type':'text/plain','Content-Length':Buffer.byteLength(managed)});
  res.end(managed);
});
server.listen(0, '127.0.0.1', () => {
  fs.writeFileSync(path + '/subscription.txt', 'http://127.0.0.1:' + server.address().port + '/feed', {mode:0o600,flag:'wx'});
});
setTimeout(() => server.close(() => process.exit(0)), 150000).unref();
JS
feed_pid=$!
for ((i=0;i<50;i++)); do [[ -f $scratch/subscription.txt ]] && break; sleep 0.1; done
[[ -f $scratch/subscription.txt ]]

stage=actual_installed_qml_bridge
test_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
QT_QPA_PLATFORM=offscreen QSG_RHI_BACKEND=software \
  OMAVLESS_R6_BRIDGE_SCRATCH="$scratch" \
  /usr/bin/quickshell --no-color --path "$test_dir/installed_native_bridge.qml" \
  >"$scratch/quickshell.log" 2>&1 &
qml_pid=$!
for ((i=0;i<1350;i++)); do
  if rg -q 'INSTALLED QML BRIDGE MATRIX PASS|BRIDGE FAIL:' "$scratch/quickshell.log"; then break; fi
  kill -0 "$qml_pid" 2>/dev/null || break
  sleep 0.1
done
stop_qml
rg -q 'INSTALLED QML BRIDGE MATRIX PASS' "$scratch/quickshell.log"
! rg -q 'BRIDGE FAIL:' "$scratch/quickshell.log"
sed -n 's/^.*\(BRIDGE PASS: [a-z_]*\).*$/\1/p' "$scratch/quickshell.log"

stage=private_export_and_real_http_evidence
for file in "$scratch/report.json" "$scratch/export.txt"; do
  [[ -f $file && ! -L $file && $(stat -c %u "$file") == "$EUID" && $(stat -c %a "$file") == 600 ]]
done
jq -e '(.schemaVersion == 2 or .schemaVersion == 3) and .scope == "native_support"' "$scratch/report.json" >/dev/null
[[ $(<"$scratch/profile.txt") == "$(<"$scratch/export.txt")" ]]
[[ $(<"$scratch/request-count") == 3 ]]
/usr/bin/omavless runtime observation >"$scratch/final.json"
jq -e '.ok==true and .result.availability=="observed" and .result.lastKnownActual=="disconnected"
  and .result.desired.connected==false and .result.manualRecoveryRequired==false
  and .result.facts.ownedCoreRunning==false and .result.facts.visibleMihomoCount==0
  and .result.facts.visibleTunCount==0 and .result.facts.ownedAuxiliaryMihomoCount==0' "$scratch/final.json" >/dev/null
printf '%s\n' 'INSTALLED BRIDGE MATRIX PASS: real Service/launcher/CLI/daemon, Python unavailable, no VPN or chooser evidence'
