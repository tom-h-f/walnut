# Walnut

<p align="center">
  <img alt="Walnut Logo" src="assets/img/WalnutComplete.svg">
</p>

Hobby x86_64 kernel. It boots as a UEFI application and exits boot services before `kmain`. I am writing it in Rust, around a full-time job and a cybersecurity degree.

The target is `x86_64-unknown-uefi`. `core` and `alloc` are built from source because that target has no prebuilt std. `.cargo/config.toml` sets the target, `build-std = ["core", "alloc"]`, and the runner `scripts/run.sh`.

`x86_64` 0.14 still expects `asm!` to be in the prelude. That stopped on nightly-2021-12-15. `rust-toolchain.toml` pins `nightly-2021-12-14`, which is the last nightly I measured that builds this tree. Do not pass `+nightly`. That selects today's nightly and the build breaks.

## What it does

`efi_main` stores the EFI system table, calls `GetMemoryMap`, then `ExitBootServices`. Serial output between those two calls is fine. A UEFI console print is not: it allocates, the map key dies, and `ExitBootServices` returns `EFI_INVALID_PARAMETER`.

Usable ranges are boot-services code and data, conventional memory, and persistent memory. Runtime services stay mapped, because `SetVirtualAddressMap` still calls into them. MMIO is devices. Loader code and data are this image. ACPI reclaim is left alone until the tables have been read.

Runtime services are relocated at physical + 10 TiB (`IDENTITY_MAP_OFFSET`). That keeps their virtual addresses off the low physical memory UEFI identity-mapped. There is no higher-half kernel map yet, so the offset stays in the lower canonical half.

ACPI comes from the UEFI configuration table. The ACPI 2.0 GUID is preferred, then 1.0. The RSDP must be revision 2 or newer: the XSDT pointer is not in the 1.0 structure. Each XSDT entry is an 8-byte physical address. A length that is not a multiple of 8 is rejected. Each table is checksummed (the sum of every byte, including the checksum byte, is 0 mod 256). The MADT parse counts local APICs with the enabled flag and records the I/O APIC address. A core marked only online-capable is not counted: it can be started later, and it is not online now.

A bump frame allocator is built over the largest usable range, and a `linked_list_allocator` heap is placed on that same range. The first frame the bump allocator returns is the page at `0x1000`, not the base of the range. The heap does not go through that allocator. `kmain` prints `KernelInfo` and one heap `Box` on COM1 (port `0x3F8`), then panics. The panic handler writes `0x11` to QEMU's `isa-debug-exit` port `0xF4`.

Paging types for 4 KiB, 2 MiB, and 1 GiB pages are in the tree, and `active_level_4_table` can read the PML4 from CR3. The kernel does not install its own page tables. A PTE's low 12 bits are flags (present, writable, user, cache, accessed, dirty, huge, global, no-execute). Those bits are handled by the `x86_64` crate. This kernel only adds the virtual offset so the PML4 frame is reachable after the runtime relocation.

`src/boot` and `src/libc` are leftover C. Cargo does not compile them, and they are not part of the boot path. There is no gnu-efi submodule.

## Not done

- GDT
- IDT and exception handlers
- Keyboard
- Threads
- Reclaiming boot-services memory in the frame allocator, and reserving MMIO and runtime regions there
- Installing a kernel page table and switching CR3
- Userspace
- Capturing x87/XMM in `SysState` (that needs an FXSAVE region)

`tests/TODO.md` lists integration tests that are not written: exception handlers, page tables, userspace programs.

## Build

Rustup picks up `rust-toolchain.toml` (nightly-2021-12-14, with `rust-src`, `clippy`, and `rustfmt`).

```bash
cargo build -Z build-std
```

`cargo build` alone also works: the same `build-std` list is in `.cargo/config.toml`. The image is `target/x86_64-unknown-uefi/debug/walnut.efi`.

There is no `Makefile.toml` and no `cargo-make` target.

## Test

Tests are UEFI binaries with a custom harness (`#[test_case]`). They do not run on the host. `cargo test` runs them through `scripts/run.sh`.

```bash
cargo test --lib
cargo test --test '*'
cargo clippy
```

Do not pass a bare `-Z build-std` to `cargo test`. That asks Cargo to build the `test` crate for this target, and this nightly rejects it (`restricted_std`). The config file already limits `build-std` to `core` and `alloc`.

On success the harness writes `0x10` to port `0xF4`. QEMU reports `(0x10 << 1) | 1` = 33, and the script exits 0.

Lib tests cover alignment, the 52-bit physical mask, frame-allocator errors (`Exhausted`, `PastRange`), and the MADT / local APIC flags. The QEMU boot tests (`should_panic`, `basic_boot`, the heap cases) need a firmware image.

## Run

`qemu-system-x86_64` and an OVMF firmware image. The script looks for:

- `/usr/share/ovmf/OVMF.fd`
- `/usr/share/OVMF/OVMF_CODE.fd`
- `/opt/homebrew/share/qemu/edk2-x86_64-code.fd`
- `/usr/local/share/qemu/edk2-x86_64-code.fd`

```bash
cargo run -Z build-std
```

The runner copies the image to `target/EFI/BOOT/BOOTx64.EFI` and boots it from a virtual FAT volume. The machine is q35, 1 GiB RAM, 4 CPUs, serial on stdio, no display. Linux uses `accel=kvm:tcg`. macOS uses `accel=tcg` (there is no KVM). `-no-reboot` is set so a triple fault does not sit in a loop.

A normal boot panics at the end of `kmain`, so QEMU exits with a failure status. That is the current end of the kernel.
