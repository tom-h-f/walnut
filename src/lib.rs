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
// Used for allocator error handling, implemented in `memory/mod.rs:66`
#![feature(alloc_error_handler)]
// Used for implementing operations for `PhysFrame` across different Page Sizes
#![feature(const_fn_trait_bound)]

extern crate alloc;

pub mod arch;
pub mod efi;
pub mod io;

pub mod memory;
use memory::allocator::FrameAllocator;
pub use memory::{PhysAddr, VirtAddr};

pub use alloc::*;
pub use alloc::boxed::Box;

pub const IDENTITY_MAP_OFFSET: u64 = (((1024 * 1024) * 1024) * 1024) * 10;


// Macros

#[macro_export]
macro_rules! whereami {
    () => {
        {
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let name = type_name_of(f);
        crate::println!("{}:{}", &name[..name.len() - 3], line!());
        }
    };
}


#[macro_export]
macro_rules! dump_stack {
    () => {
        walnut::println!("Dumping 1024 bytes from stack: ");
        // Get stack pointer
        let rsp = walnut::arch::register::read("rsp").unwrap();
        // Read 1024 bytes from stack pointer
        let data = unsafe {
            *(rsp as *const [u8; 1024])
        };
        for i in (0..data.len()).step_by(16) {
            for j in 0..16 {
                walnut::print!("{:02X?} ", data[i+j]);
                if j == 15 {
                    walnut::print!("\n");
                } else if j == 7 {
                    walnut::print!("\t");
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct KernelInfo {
    pub memory_map: memory::RangeSet,
    pub frame_allocator: FrameAllocator,
}


//TODO pub static KINFO: spin::Mutex<arc

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
        print!("{:<48}", core::any::type_name::<T>());
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
pub unsafe extern "efiapi" fn efi_main(
    handle: efi::structures::EfiHandle,
    st: *mut efi::structures::EfiSystemTable,
) -> u64 {
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
    assert_eq!(1, 1);
}

#[test_case]
fn vec_push() {
    use alloc::vec::Vec;
    let mut v: Vec<u64> = Vec::new();
    for i in 0..200 {
        //v.push(0x41);
        //v.pop();
    }
}

