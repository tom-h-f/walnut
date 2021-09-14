# Walnut OS
<p align="center">
  <img alt="Walnut Logo" src="assets/img/WalnutComplete.svg">
</p>




This is a hobby OS developed very slowly as I attempt to work on outside of a fulltime job and fulltime studying for a cybersecurity degree.

Currently only plan to support UEFI OS's, which, along with Rust brilliant cross compiling abilities, allows me to support all modern hardware (x86_64, ARM, RISC-V, etc).

## TODOs

- [ ] Exit UEFI boot services, into OS management.
  - [ ] Create `fn` in `efi.rs` to perform the exit, aswell as starting the OS's memory manager before hand for a safe transition.
  - [ ] `efi::get_memory_map()` causes a `#PG` as the `boot_services` pointer is always `0xafafafafafaf`, i dont know why, my structs seem correct but yano.


