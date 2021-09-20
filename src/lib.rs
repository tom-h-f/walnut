#![no_std]
// Configuration to enable running our custom test setup
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


// Needed for `efi_main` calling convention
#![feature(abi_efiapi)]
// inline assembly for libcore requirement functions, e.g memcpy
#![feature(asm)]
// Map bools to options, used at `efi/mod.rs:313`
#![feature(bool_to_option)]
// Used in the linked list allocator in `memory/paging/linked_list.rs`
#![feature(const_mut_refs)]

extern crate alloc;


pub mod io;
pub mod arch;
pub mod efi;
pub mod memory;






































// --------------------------------------------------
// Testing
// --------------------------------------------------

use core::panic::PanicInfo;

/// Custom Test trait for the testing setup
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

/// Run each test, then exit QEMU with success exit code
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit_success();
}



/// Panic handler for the test harness
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    qemu::exit_failed();
    unreachable!();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(handle: efi::structures::EfiHandle, 
                                       st: *mut efi::structures::EfiSystemTable) -> u64 {
    
    efi::init(&mut *st).expect("Couldnt intialize EFI structures");
    efi::exit_boot_services(handle).unwrap();
    test_main();
    loop {}
}

/// Call the test panic_handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
