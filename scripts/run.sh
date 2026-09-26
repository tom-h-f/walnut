#!/bin/bash
# Boot a UEFI image in QEMU and return the isa-debug-exit status.
#
# The kernel writes a u32 to port 0xF4. QEMU turns that into
# (value << 1) | 1. Success is 0x10, which is process status 33, and
# this script exits 0 for that status. Any other status is passed through.

set -u
set -o pipefail

DEFAULT_BIN_PATH="target/x86_64-unknown-uefi/debug/walnut.efi"
BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"

if [ $# -ge 1 ] && [ -n "$1" ]; then
  BIN_PATH="$1"
else
  BIN_PATH="$DEFAULT_BIN_PATH"
fi

if [ ! -f "$BIN_PATH" ]; then
  echo "EFI image not found: $BIN_PATH" >&2
  exit 1
fi

if ! command -v qemu-system-x86_64 >/dev/null 2>&1; then
  echo "qemu-system-x86_64 is not installed" >&2
  exit 1
fi

# Linux can use KVM and fall back to TCG. Darwin has no KVM.
case "$(uname -s)" in
  Darwin) ACCEL="tcg" ;;
  *) ACCEL="kvm:tcg" ;;
esac

OVMF=""
for candidate in \
  /usr/share/ovmf/OVMF.fd \
  /usr/share/OVMF/OVMF_CODE.fd \
  /opt/homebrew/share/qemu/edk2-x86_64-code.fd \
  /usr/local/share/qemu/edk2-x86_64-code.fd
do
  if [ -f "$candidate" ]; then
    OVMF="$candidate"
    break
  fi
done

if [ -z "$OVMF" ]; then
  echo "OVMF firmware not found (tried /usr/share/ovmf/OVMF.fd and Homebrew edk2-x86_64-code.fd)" >&2
  exit 1
fi

mkdir -p "$BASE_DIR/target/EFI/BOOT"
cp "$BIN_PATH" "$BASE_DIR/target/EFI/BOOT/BOOTx64.EFI"

echo "Running QEMU..."

# One CPU. The kernel has no AP startup and no IDT, so extra CPUs fault
# during ExitBootServices and QEMU resets before serial output.
# Serial goes to a file: isa-debug-exit tears QEMU down without flushing
# a stdio pipe, which used to swallow the kernel log.
SERIAL_LOG="${BASE_DIR}/target/serial.log"
rm -f "$SERIAL_LOG"
qemu-system-x86_64 \
  -nodefaults \
  -machine "q35,accel=${ACCEL}" \
  -m 1G \
  -drive "if=pflash,format=raw,readonly=on,file=${OVMF}" \
  -drive "format=raw,file=fat:rw:${BASE_DIR}/target/" \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -serial "file:${SERIAL_LOG}" \
  -smp 1 \
  -nographic \
  -no-reboot
status=$?

if [ -f "$SERIAL_LOG" ]; then
  grep -v "BdsDxe" "$SERIAL_LOG" || true
fi

if [ "$status" -eq 33 ]; then
  exit 0
fi
exit "$status"
