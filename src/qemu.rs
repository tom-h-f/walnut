#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit(exit_code: ExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // We set the QEMU exit code via writing a 4-bit int to
        // an MMIO port at 0xf4
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
