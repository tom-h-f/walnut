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

use walnut::{
    efi::{
        self,
        structures::{EfiHandle, EfiSystemTable},
    },
    memory, println, KernelInfo,
};

/// Entry point of that UEFI calls.
///
/// Gets the memory map from EFI, and exits UEFI Boot Services
///
/// # Safety
/// Can be unsafe due to accessing structures and functions
/// from raw physical memory.
#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(handle: EfiHandle, st: *mut EfiSystemTable) -> u64 {
    efi::init(&mut *st).expect("Couldn't intialize UEFI");
    let memory_map = efi::exit_boot_services(handle).expect("Unable to exit UEFI boot services");

    // Intialize ACPI Tables
    efi::acpi::init().expect("Couldn't intialize ACPI");
    let frame_allocator = memory::init(
        *memory_map
            .largest()
            .expect("Couldn't get largest memory range"),
    )
    .expect("Couldn't intialize frame allocator");

    // Create kernel_info to pass into the kernel main
    let kinfo = KernelInfo {
        memory_map,
        frame_allocator,
    };

    // Call kernel main and supply the memory range obtained from
    // GetMemoryMap
    kmain(kinfo);
    unreachable!();
}

/// Entry point of the kernel
pub fn kmain(mut kinfo: KernelInfo) {
    #[cfg(test)]
    test_main();

    println!("{:#X?}", kinfo);

    let frame = kinfo
        .frame_allocator
        .alloc_frame()
        .expect("frame allocator had no pages");
    println!(
        "frame {:#x} ({} allocated)",
        frame.start_address().as_u64(),
        kinfo.frame_allocator.frames_allocated()
    );

    let a = walnut::Box::new(41);
    println!("heap box {}", a);

    qemu::exit_success();
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
    // Exit QEMU
    qemu::exit_failed();
    unreachable!()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    walnut::test_panic_handler(info)
}
