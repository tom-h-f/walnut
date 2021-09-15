#[repr(C)]
pub struct Port {
   pub devices: [Option<u16>; 4],
}

impl Port {
    pub unsafe fn new(bios_base: *const u16) -> Self {
        let mut ret = Port {
            devices: [None; 4],
        };
        for (i, dev) in ret.devices.iter_mut().enumerate() {
            let port = *bios_base.offset(i as isize);

            // If port address is zero, it is being reported as 
            // not present by the BIOS
            if port == 0 {
                *dev = None;
                continue;
            }

            // Initialize the serial port to a known state
            outb(port + 1, 0x00); // Disable all interrupts
            outb(port + 3, 0x80); // Enable DLAB
            outb(port + 0, 0x01); // Low byte divisor (115200 baud)
            outb(port + 1, 0x00); // High byte divisor
            outb(port + 3, 0x03); // 8 bits, 1 stop bit, no parity
            outb(port + 4, 0x03); // RTS/DSR set
            *dev = Some(port)
        }
        ret
    }
    
     /// Read a byte from whatever COM port has a byte available
    pub fn read_byte(&mut self) -> Option<u8> {
        // Go through each device
        for port in &self.devices {
            // If the device is present
            if let &Some(port) = port {
                unsafe {
                    // Check if there is a byte available
                    if (inb(port + 5) & 1) == 0 {
                        // No byte available
                        continue;
                    }

                    // Read the byte that was present on this port
                    return Some(inb(port));
                }
            }
        }

        // No bytes available
        None
    }

    /// Write a byte to a COM port
    fn write_byte(&mut self, port: usize, byte: u8) {
        // Write a CR prior to all LFs
        if byte == b'\n' { self.write_byte(port, b'\r'); }

        // Check if this COM port exists
        if let Some(&Some(port)) = self.devices.get(port) {
            unsafe {
                // Wait for the output buffer to be ready
                while (inb(port + 5) & 0x20) == 0 {}

                // Write the byte!
                outb(port, byte);
            }
        }
    }

    /// Write bytes to all known serial devices
    pub fn write(&mut self, bytes: &[u8]) {
        // Go through each byte
        for &byte in bytes {
            // Broadcast the byte to all present devices
            for com_id in 0..self.devices.len() {
                self.write_byte(com_id, byte);
            }
        }
    }


    
}

#[inline]
pub unsafe fn outb(addr: u16, val: u8) {
    asm!("out {0}, al", in(reg) addr as usize, in("al") val);
}

#[inline]
pub unsafe fn inb(addr: u16) -> u8 {
    let val: u8;
    llvm_asm!("in al, dx" : "={al}"(val) : "{dx}"(addr) :: "volatile", "intel");
    val
}

