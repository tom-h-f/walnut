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

#[cfg(debug_assertions)]
const VERBOSE: bool = true;
#[cfg(not(debug_assertions))]
const VERBOSE: bool = false;

use efi::{EfiHandle, EfiStatus, EfiSystemTable};

#[no_mangle]
extern "C" fn efi_main(_handle: EfiHandle, st: *mut EfiSystemTable) -> EfiStatus {
    unsafe {
        efi::register_system_table(st);
    }

    for i in 0..20 {
        print!(".");
        if i == 9 {
        }
    }
    EfiStatus(0)
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
