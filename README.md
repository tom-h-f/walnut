# Walnut OS
<p align="center">
  <img alt="Walnut Logo" src="assets/img/WalnutComplete.svg">
</p>

I am discountining this branch tsince allocing is fucking impossible with UEFI...
I will just do BIOS 
Keeping this branch alive in case i want to come back for bits of code however.




This is a hobby OS developed very slowly as I attempt to work on outside of a fulltime job and fulltime studying for a cybersecurity degree.

Currently only plan to support UEFI OS's, which, along with Rust brilliant cross compiling abilities, allows me to support all modern hardware (x86_64, ARM, RISC-V, etc).

## Dependencies

0. QEMU
1. rustc
2. [`cargo-make`](https://github.com/sagiegurari/cargo-make) (Install with `cargo install cargo-make`)


## Running the OS

```bash
cargo make run
```
## Roadmap

### Paging

1. Exit boot services with the UEFI memory map.
2. Setup a physical memory allocator with that map. Reclaim boot services and loader regions in the future. Platform runtime regions should be kernel-reserved. I guess MMIO regions should also be reserved.
3. Identity map the lower 4GB, and map anything else you want. Maybe the framebuffer and higher-half kernel too. Let's hope UEFI doesn't have its NMI handler beyond that.
4. Alternatively, I should probably identity map everything in the memory map, but I don't know if there's any guarantee that the higher-half will be left untouched.
5. Do your CPU setup. GDT, IDT, ... Disable/flush caching, change MTRR+PAT registers, switch CR3, then re-enable caching.
6. The physical memory manager could use a physical memory mirror for now on.


- [ ] ACPI
- [ ] Serial Driver
- [ ] GDT
- [ ] Paging/Memory Management
- [ ] Interrupts/IDT
- [ ] Multithreading support
- [ ] Keyboard Driver

