#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_dispatcher)]
#![reexport_test_harness_main = "test_main"]

pub mod graphics;
pub mod qemu;
pub mod io;
pub mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    #[cfg(test)]
    test_main();

    loop {}
}



#[cfg(test)]
fn test_dispatcher(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests...", tests.len());
    for test in tests {
        test();
    }
    qemu::exit(qemu::ExitCode::Success);
}

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}



#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
