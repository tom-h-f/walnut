//! Integration test that shoddily replicates a test that succeeds
//! on failure.
#![no_std]
#![no_main]

extern crate qemu;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn efi_main() -> ! {
    should_fail();
    walnut::println!("[test did not panic]");
    qemu::exit_failed();
    loop {}
}

fn should_fail() {
    walnut::print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    walnut::print!("[ok]");
    qemu::exit_success();
    loop {}
}
