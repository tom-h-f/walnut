#![no_std]
#![no_main]
#![feature(asm, abi_efiapi, llvm_asm, bool_to_option)]
#![feature(panic_info_message)]
#[doc(hidden)]
mod core_fns;
#[doc(hidden)]
mod efi;
pub mod serial;
pub mod acpi;
pub mod mm;
pub mod cpu;
pub mod register;

use efi::{EfiHandle, EfiStatus, EfiSystemTable};


pub fn kmain() {
    dump_state!();
    
    panic!("reached end of kmain()");
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    /*println!("!!! PANIC !!!");
    if let Some(loc) = _info.location() {
        println!("Location: {}", loc);
    }
    if let Some(msg) = _info.message() {
        println!("Message:  {}", msg);
    }*/
    loop {}
}
