#![no_std]
#![no_main]
// Enable testing
#![feature(custom_test_frameworks)]
#![test_runner(walnut::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Detailed information for panics
#![feature(panic_info_message)]
// Needed for `efi_main` calling convention
#![feature(abi_efiapi)]

use walnut::
{
    efi::{
        self,
        structures::{EfiHandle, EfiSystemTable},
    }, 
    memory::RangeSet,
} ;

/// Entry point of that UEFI calls.
///
/// Gets the memory map from EFI, and exits UEFI Boot Services
///
/// # Safety
/// Can be unsafe due to accessing structures and functions
/// from raw physical memory.
#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(handle: EfiHandle, st: *mut EfiSystemTable) -> u64 {
    efi::init(&mut *st).expect("Couldnt intialize EFI structures");
    let memory_range = efi::exit_boot_services(handle).expect("Unable to exit UEFI boot services");

    // Run tests after we exit UEFI boot services
    #[cfg(test)]
    test_main();

    // Call kernel main and supply the memory range obtained from
    // GetMemoryMap
    kmain(memory_range);
    unreachable!();
}

/// Entry point of the kernel
pub fn kmain(memory_range: RangeSet)  {
    walnut::println!("{:#x?}", memory_range);
    walnut::println!("{:#x?}", memory_range.total_size());
    walnut::println!("Largest: {:#x?}", memory_range.largest().unwrap());

    panic!("reached end of kmain")
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    walnut::println!("!!! PANIC !!!");
    if let Some(loc) = _info.location() {
        walnut::println!("Location: {}", loc);
    }
    if let Some(msg) = _info.message() {
        walnut::println!("Message:  {}", msg);
    }
    walnut::dump_state!();
    // Exit QEMU
    qemu::exit_failed();
    unreachable!()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    walnut::test_panic_handler(info)
}
