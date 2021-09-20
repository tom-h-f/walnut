#!/bin/bash

DEFAULT_BIN_PATH="target/x86_64-unknown-uefi/debug/walnut.efi"


if [ $# -eq 0 ]; then
  if [ ! -f DEFAULT_BIN_PATH ]; then
    echo "Please provide a binary name as an argument"
    exit 1
  else
    BIN_PATH=$DEFAULT_BIN_PATH
  fi
else
  BIN_PATH=$1
fi

if [ "$1" = "" ]; then
  BIN_PATH=$DEFAULT_BIN_PATH
fi

mkdir -p target/EFI/BOOT
cp $BIN_PATH target/EFI/BOOT/BOOTx64.EFI

echo "Running QEMU..."

# Preserve exit code before we pipe to sed 
# Then remove VT100 Escape Codes
# And remove QEMU UEFI output
set -o pipefail
qemu-system-x86_64 \
  -enable-kvm \
  -nodefaults \
  -vga std \
  -machine q35,accel=kvm:tcg \
  -m 1G \
  -drive if=pflash,format=raw,readonly=true,file=/usr/share/ovmf/OVMF.fd \
  -drive format=raw,file=fat:rw:./target/ \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -serial stdio \
  -smp 4 \
  -nographic | \
  $2           \
  grep -v "BdsDxe" # Removes useless QEMU UEFI output and strips VT100 colour codes


# Check for the desired QEMU exit code, and exit with 0
# Else, exit with the original code
[ $? -eq 33 ] && exit 0 || exit $QEMU_EXITCODE 
