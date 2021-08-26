#![no_std]
#![no_main]

pub mod graphics;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello {}", "World");



    loop {}
}




/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
