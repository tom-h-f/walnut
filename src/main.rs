#![no_std]
#![no_main]
#![feature(asm, abi_efiapi)]
#![feature(panic_info_message)]
#[doc(hidden)]
mod core_fns;
#[doc(hidden)]
mod efi;
#[doc(hidden)]
#[macro_use]
mod print;
pub mod kernel;

use efi::{EfiHandle, EfiStatus, EfiSystemTable};

#[no_mangle]
extern "C" fn efi_main(_handle: EfiHandle, st: *mut EfiSystemTable) -> EfiStatus {
    unsafe {
        efi::register_system_table(st);
        println!("Loaded UEFI System Table at {:p}", st);

        println!("Exiting Boot services...");
        efi::exit_boot_services(_handle);
    }



    panic!("efi_main exited");
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    println!("!!! PANIC !!!");
    if let Some(loc) = _info.location() {
        println!("Location: {}", loc);
    }
    if let Some(msg) = _info.message() {
        println!("Message:  {}", msg);
    }
    loop {}
}
