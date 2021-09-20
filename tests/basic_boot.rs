#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(walnut::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn efi_main() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    walnut::test_panic_handler(info)
}

// This case tests that we can print before caling any EFI routines
// as we should be able to write to the IO port before we intialize the EfiSystemTable etc.
#[test_case]
fn test_println() {
    walnut::println!("test_println output");
}

