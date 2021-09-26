# Walnut OS
<p align="center">
  <img alt="Walnut Logo" src="assets/img/WalnutComplete.svg">
</p>

This is a hobby OS developed very slowly as I attempt to work on outside of a fulltime job and fulltime studying for a cybersecurity degree.

## Dependencies

0. GCC
1. Make
3. QEMU


## Running the OS

```bash
# Clone the git repository, along with the GNU-EFI submodule
git clone --recurse-submodules https://github.com/orsetii/walnut

# Compile GNU-EFI, kernel, bootloader and wlibc 
make all -j$(nproc)
make -j$(nproc)

# Run in QEMU (will fail if qemu-system-x86_64 is not installed)
make run
```

## TODOs

- [ ] Working printf, currently snprintf attempts a `__chkstk` call which causes it to be fucked. So if you call `efi_println` and then `efi_snprintf` it will half print it, but if you call just `efi_snprintf` it prints nothing.
