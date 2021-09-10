#!/bin/bash
set -e



qemu-system-x86_64 \
  -enable-kvm \
  -m 512     \
  -nographic \
  -bios /usr/share/ovmf/OVMF.fd \
  -device driver=e1000,netdev=n0 \
  -netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=walnut.efi \
  $EXTRAFLAGS
