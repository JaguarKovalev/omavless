#!/bin/bash
# TEST TOOL ONLY. Temporarily restricts Python execution across THIS VM.
# Never install this as an application helper or invoke without attended review.
set -euo pipefail
export PATH=/usr/bin:/bin LC_ALL=C
umask 077
target=/usr/bin/python3.14
session=/run/omavless-r6-python-mask

refuse() { printf '%s\n' 'PYTHON_MASK_REFUSED'; exit 2; }
[[ $EUID == 0 && $# -ge 1 ]] || refuse
[[ $(readlink /proc/self/ns/mnt) == "$(readlink /proc/1/ns/mnt)" &&
   $(readlink /proc/self/ns/user) == "$(readlink /proc/1/ns/user)" ]] || refuse

process_start() {
  local line rest
  local -a fields
  [[ $1 =~ ^[1-9][0-9]*$ && -r /proc/$1/stat ]] || return 1
  line=$(</proc/"$1"/stat)
  rest=${line##*) }
  read -ra fields <<<"$rest"
  [[ ${fields[19]:-} =~ ^[0-9]+$ ]] || return 1
  printf '%s' "${fields[19]}"
}
mount_id() {
  local ids
  # Read the kernel's exact fixed mountpoint record. A query error must never
  # look like "not mounted", including in a [[ -z $(mount_id) ]] check.
  if ! ids=$(awk -v target="$target" '$5 == target { print $1 }' /proc/self/mountinfo 2>/dev/null); then
    printf '%s' 'MOUNT_QUERY_FAILED'
    return 1
  fi
  printf '%s' "$ids"
}
identity() { stat -c '%d:%i:%u:%a:%s' -- "$1"; }
safe_session() {
  [[ -d $session && ! -L $session && $(stat -c '%u:%a' "$session") == 0:700 &&
     -f $session/mask && ! -L $session/mask && $(stat -c '%u:%a:%s' "$session/mask") == 0:0:0 &&
     -f $session/original && ! -L $session/original && $(stat -c '%u:%a' "$session/original") == 0:600 &&
     $(stat -c %s "$session/original") -le 256 ]]
}

# Main guardian and independent watchdog serialize restoration. Only our unique
# root-owned mask inode and captured mount ID authorize the fixed unmount.
restore() (
  safe_session || exit 1
  exec 8>"$session/restore.lock"
  flock -x 8
  local old_identity old_digest current_id recorded_id
  read -r old_identity old_digest <"$session/original"
  [[ $old_identity =~ ^[0-9]+:[0-9]+:0:755:[0-9]+$ && $old_digest =~ ^[0-9a-f]{64}$ ]] || exit 1
  current_id=$(mount_id)
  if [[ -n $current_id ]]; then
    [[ $current_id =~ ^[1-9][0-9]*$ && $(identity "$target") == "$(identity "$session/mask")" ]] || exit 1
    if [[ -f $session/mount-id ]]; then
      read -r recorded_id <"$session/mount-id"
      [[ $recorded_id == "$current_id" ]] || exit 1
    else
      # The parent can die between bind and writing its receipt. The unique
      # mask inode proves that mount; capture its exact ID before unmounting.
      printf '%s\n' "$current_id" >"$session/mount-id"
    fi
    [[ $(mount_id) == "$current_id" ]] || exit 1
    umount -- "$target" >/dev/null 2>&1 || exit 1
  fi
  [[ -z $(mount_id) && $(identity "$target") == "$old_identity" &&
     $(sha256sum "$target" | cut -d' ' -f1) == "$old_digest" ]] || exit 1
  : >"$session/restored"
)

if [[ $1 == --watchdog ]]; then
  [[ $# == 1 ]] || refuse
  safe_session || refuse
  [[ $(readlink -f "${BASH_SOURCE[0]}") == "$session/guardian.sh" &&
     $(stat -c '%u:%a' "$session/guardian.sh") == 0:700 ]] || refuse
  read -r guardian_pid guardian_start <"$session/guardian"
  [[ $guardian_pid =~ ^[1-9][0-9]*$ && $guardian_start =~ ^[0-9]+$ ]] || refuse
  # Independent session survives parent death and foreground-terminal signals.
  # Deadline restores Python ONLY; it does not stop VPN or expire a human prompt.
  : >"$session/watchdog-ready"
  started=$SECONDS
  while [[ ! -e $session/restored ]]; do
    if [[ $(process_start "$guardian_pid" || true) != "$guardian_start" ]] || ((SECONDS-started >= 900)); then
      if ! restore; then printf '%s\n' 'PYTHON_MASK_RECOVERY_REQUIRED' >"$session/recovery-required"; fi
      exit 0
    fi
    sleep 1
  done
  exit 0
fi

[[ $1 == --run && $# == 3 && $2 =~ ^[0-9a-f]{64}$ && $3 =~ ^[1-9][0-9]*$ && ! -t 0 ]] || refuse
expected=$2
driver_pid=$3
[[ ! -e $session && ! -L $session && ! -e /var/lib/pacman/db.lck &&
   -f $target && ! -L $target && $(stat -c '%u:%a:%h' "$target") == 0:755:1 &&
   $(readlink -f "$target") == "$target" && -z $(mount_id) &&
   $(pacman -Qqo "$target" 2>/dev/null) == python &&
   $(sha256sum "$target" | cut -d' ' -f1) == "$expected" ]] || refuse
[[ $(stat -Lc '%d:%i' "/proc/$driver_pid/exe") == "$(stat -c '%d:%i' "$target")" ]] || refuse
# Do not interrupt another currently executing Python workload. Recheck as root;
# only the already-loaded, deliberately participating driver is permitted.
for comm_file in /proc/[0-9]*/comm; do
  [[ -r $comm_file ]] || continue
  process_name=$(<"$comm_file") || continue
  if [[ $process_name =~ ^python([0-9]+(\.[0-9]+)*)?$ ]]; then
    candidate=${comm_file#/proc/}; candidate=${candidate%/comm}
    [[ $candidate == "$driver_pid" ]] || refuse
  fi
done

mkdir -m0700 "$session" || refuse
install -m000 /dev/null "$session/mask"
printf '%s %s\n' "$(identity "$target")" "$expected" >"$session/original"
guardian_pid=$BASHPID
printf '%s %s\n' "$guardian_pid" "$(process_start "$guardian_pid")" >"$session/guardian"
install -m0700 "${BASH_SOURCE[0]}" "$session/guardian.sh"
watchdog_pid=
cleanup() {
  local result=$?
  trap - EXIT HUP INT TERM
  if restore; then
    printf '%s\n' 'PYTHON_MASK_RESTORED'
    # Wait for only our direct watchdog; never kill it before restoration.
    if [[ -n $watchdog_pid ]]; then wait "$watchdog_pid" 2>/dev/null || true; fi
    # Preserve a root-owned failure record; success can remove exact known files.
    for leaf in mask original guardian guardian.sh watchdog-ready mount-id restored restore.lock; do
      [[ ! -e $session/$leaf && ! -L $session/$leaf ]] || unlink "$session/$leaf"
    done
    rmdir "$session" || result=1
  else
    printf '%s\n' 'PYTHON_MASK_RECOVERY_REQUIRED'
    result=1
  fi
  exit "$result"
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM
setsid /bin/bash "$session/guardian.sh" --watchdog </dev/null >/dev/null 2>&1 &
watchdog_pid=$!
for ((i=0;i<100;i++)); do [[ -f $session/watchdog-ready ]] && break; sleep 0.02; done
[[ -f $session/watchdog-ready ]] || exit 1
arm_mask() (
  trap - EXIT HUP INT TERM
  exec 8>"$session/restore.lock"
  flock -x 8
  # A stopped parent cannot mount after its watchdog has already expired.
  [[ ! -e $session/restored && -z $(mount_id) ]] || exit 1
  kill -0 "$watchdog_pid" 2>/dev/null || exit 1
  mount --bind "$session/mask" "$target" >/dev/null 2>&1
  current_id=$(mount_id)
  [[ $current_id =~ ^[1-9][0-9]*$ && $(identity "$target") == "$(identity "$session/mask")" ]] || exit 1
  printf '%s\n' "$current_id" >"$session/mount-id"
)
arm_mask
printf '%s\n' 'PYTHON_MASK_READY'
# EOF is the parent-death lifeline. No arbitrary commands are accepted.
while [[ ! -e $session/restored ]]; do
  if IFS= read -r -t 1 -n 16 release; then
    [[ $release == release ]] || exit 1
    exit 0
  else
    read_status=$?
    [[ $read_status -gt 128 ]] || exit 0
  fi
done
# The watchdog restored at its deadline: fail the acceptance, not the machine.
exit 1
