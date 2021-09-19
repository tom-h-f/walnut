# Walnut OS
<p align="center">
  <img alt="Walnut Logo" src="assets/img/WalnutComplete.svg">
</p>




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

- [ ] ACPI
- [ ] Serial Driver
- [ ] GDT
- [ ] Paging/Memory Management
- [ ] Interrupts/IDT
- [ ] Multithreading support
- [ ] Keyboard Driver




