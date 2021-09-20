#![no_std]
#![feature(asm)]

const QEMU_EXIT_IO_PORT: u16 = 0xF4;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}


/// Exits qemu with `exit_code` via writing to the port address for
/// the `isa-debug-exit` device, which must be set in the QEMU launch flags.
/// If we fail to exit, a panic occurs.
pub fn exit_qemu(exit_code: QemuExitCode, port_addr: Option<u16>) {
    unsafe {
        // Write to the provided port or the default if not provided
        write_to_port(port_addr.unwrap_or(QEMU_EXIT_IO_PORT), exit_code as u32)
    }
    panic!("QEMU did not exit after `exit_qemu` call
           Please add the `isa-debug-exit` device to your QEMU flags");
}


unsafe fn write_to_port(port: u16, val: u32) {
    asm!("out dx, eax", in("dx") port, in("eax") val, options(nomem, nostack, preserves_flags));
}
