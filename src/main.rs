#![no_std]
#![no_main]

// Enable testing 
#![feature(custom_test_frameworks)]
#![test_runner(walnut::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(panic_info_message)]
// Needed for `efi_main` calling convention
#![feature(abi_efiapi)]

use walnut::println;
use core::panic::PanicInfo;

/// Entry point of that UEFI calls.
///
/// Gets the memory map from EFI, and exits UEFI Boot Services
///
/// # Safety 
/// Can be unsafe due to accessing structures and functions
/// from raw physical memory.
#[no_mangle]
pub unsafe extern "efiapi" fn efi_main() -> u64 {

    kmain();
    unreachable!();
}

pub fn kmain() {

    #[cfg(test)]
    test_main();
    #[cfg(test)]
    panic!("TEST");

    panic!("reached end of kmain")
}


#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    println!("!!! PANIC !!!");
    if let Some(loc) = _info.location() {
        println!("Location: {}", loc);
    }
    if let Some(msg) = _info.message() {
        println!("Message:  {}", msg);
    }
    // Exit QEMU
    qemu::exit_qemu(qemu::QemuExitCode::Failed, None);
    unreachable!()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    walnut::test_panic_handler(info)
}

