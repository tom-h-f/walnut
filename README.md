# Walnut OS
<p align="center">
  <img alt="Walnut Logo" src="assets/img/WalnutComplete.svg">
</p>




This is a hobby OS developed very slowly as I attempt to work on outside of a fulltime job and fulltime studying for a cybersecurity degree.

Currently only plan to support UEFI OS's, which, along with Rust brilliant cross compiling abilities, allows me to support all modern hardware (x86_64, ARM, RISC-V, etc).

## Blockers

- Unable to write to output via serial, going to parse ACPI table as per https://youtu.be/2YAgDJTs9So?t=13247 and then launch out of UEFI space so we can maintain output and ensure easier debugging during boot process.

## Roadmap

- [ ] ACPI
- [ ] Serial Driver
- [ ] GDT
- [ ] Paging/Memory Management
- [ ] Interrupts/IDT
- [ ] Multithreading support
- [ ] Keyboard Driver




