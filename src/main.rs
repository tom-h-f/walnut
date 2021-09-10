#![no_std]
#![no_main]

#[no_mangle]
extern "C" fn efi_main() {}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
