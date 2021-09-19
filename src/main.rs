#![no_std]
#![no_main]
#![feature(asm, abi_efiapi, llvm_asm, bool_to_option)]
#![feature(panic_info_message)]
#![allow(clippy::missing_safety_doc)]
pub mod acpi;
#[doc(hidden)]
mod core_fns;
pub mod cpu;
#[doc(hidden)]
mod efi;
mod error;
pub mod mm;
pub mod register;
pub mod serial;



pub fn kmain(memory_map: mm::rangeset::RangeSet) {
    dump_state!();
    serial_println!("Welcome to Walnut!");
    serial_println!("{:#X?}", memory_map);
    serial_println!("Total Memory: {} MiB", memory_map.total_size() / 1024 /1024);


    panic!("reached end of kmain()");
}


#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    efi_println!("!!! PANIC !!!");
    if let Some(loc) = _info.location() {
        efi_println!("Location: {}", loc);
    }
    if let Some(msg) = _info.message() {
        efi_println!("Message:  {}", msg);
    }
    loop {}
}
