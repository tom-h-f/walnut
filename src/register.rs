//! Functions to read and write from registers.

#[macro_export]
macro_rules! dump_state {
    () => {
        use crate::println;
        let s = $crate::register::SysState::new().unwrap();
        println!("{:#016x?}", s);
    };
}

/// Defines a system's state, contains key register values
pub struct SysState {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rsp: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rflags: RFlags,
}

use core::fmt;

impl fmt::Debug for SysState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         f.write_fmt(format_args!("
RAX: {:#016X?}\tRBX: {:#016X?}\n
RCX: {:#016X?}\tRDX: {:#016X?}\n
RSI: {:#016X?}\tRDI: {:#016X?}\n
RSP: {:#016X?}\tRBP: {:#016X?}\n
R8:  {:#016X?}\tR9:  {:#016X?}\n
R10: {:#016X?}\tR11: {:#016X?}\n
R12: {:#016X?}\tR13: {:#016X?}\n
R14: {:#016X?}\tR15: {:#016X?}\n
RFLAGS: {:#?}\n", 
                     self.rax, self.rbx, self.rcx, 
                     self.rdx, self.rsi, self.rdi,
                     self.rsp, self.rbp, self.r8, 
                     self.r9,  self.r10, self.r11, 
                     self.r12, self.r13, self.r14,
                     self.r15, self.rflags
                     )) // TODO do floating point and other registers
    }
}

impl SysState {
    pub fn new() -> Option<Self> {
        Some(Self {
            rax: read("rax")?,
            rbx: read("rbx")?,
            rcx: read("rcx")?,
            rdx: read("rdx")?,
            rsi: read("rsi")?,
            rdi: read("rdi")?,
            rsp: read("rsp")?,
            rbp: read("rbp")?,
            r8: read("r8")?,
            r9: read("r9")?,
            r10: read("r10")?,
            r11: read("r11")?,
            r12: read("r12")?,
            r13: read("r13")?,
            r14: read("r14")?,
            r15: read("r15")?,
            rflags: RFlags::read(),
        })
    }
}

/// Reads `T` from register `name`. If unable to read for whatever reason,
/// returns `None`, otherwise `Some(T)`
///
/// This function is safe as we only perform this operation on known register names
pub fn read(name: &str) -> Option<u64> {
    unsafe {
        match name {
            "rax" => {
                let val: u64;
                asm!("mov {0}, rax", out(reg) val);
                Some(val)
            }
            "rbx" => {
                let val: u64;
                asm!("mov {0}, rbx", out(reg) val);
                Some(val)
            }
            "rcx" => {
                let val: u64;
                asm!("mov {0}, rcx", out(reg) val);
                Some(val)
            }
            "rdx" => {
                let val: u64;
                asm!("mov {0}, rdx", out(reg) val);
                Some(val)
            }
            "rsi" => {
                let val: u64;
                asm!("mov {0}, rsi", out(reg) val);
                Some(val)
            }
            "rdi" => {
                let val: u64;
                asm!("mov {0}, rdi", out(reg) val);
                Some(val)
            }
            "rsp" => {
                let val: u64;
                asm!("mov {0}, rsp", out(reg) val);
                Some(val)
            }
            "rbp" => {
                let val: u64;
                asm!("mov {0}, rbp", out(reg) val);
                Some(val)
            }
            "r8" => {
                let val: u64;
                asm!("mov {0}, r8", out(reg) val);
                Some(val)
            }
            "r9" => {
                let val: u64;
                asm!("mov {0}, r9", out(reg) val);
                Some(val)
            }
            "r10" => {
                let val: u64;
                asm!("mov {0}, r10", out(reg) val);
                Some(val)
            }
            "r11" => {
                let val: u64;
                asm!("mov {0}, r11", out(reg) val);
                Some(val)
            }
            "r12" => {
                let val: u64;
                asm!("mov {0}, r12", out(reg) val);
                Some(val)
            }
            "r13" => {
                let val: u64;
                asm!("mov {0}, r13", out(reg) val);
                Some(val)
            }
            "r14" => {
                let val: u64;
                asm!("mov {0}, r14", out(reg) val);
                Some(val)
            }
            "r15" => {
                let val: u64;
                asm!("mov {0}, r15", out(reg) val);
                Some(val)
            }
            _ => None,
        }
    }
}

use bitflags::bitflags;

bitflags! {
    /// The RFLAGS register.
    pub struct RFlags: u64 {
        /// Processor feature identification flag.
        ///
        /// If this flag is modifiable, the CPU supports CPUID.
        const ID = 1 << 21;
        /// Indicates that an external, maskable interrupt is pending.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        /// Virtual image of the INTERRUPT_FLAG bit.
        ///
        /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
        /// interrupts (CR4.PVI) are activated.
        const VIRTUAL_INTERRUPT = 1 << 19;
        /// Enable automatic alignment checking if CR0.AM is set. Only works if CPL is 3.
        const ALIGNMENT_CHECK = 1 << 18;
        /// Enable the virtual-8086 mode.
        const VIRTUAL_8086_MODE = 1 << 17;
        /// Allows to restart an instruction following an instrucion breakpoint.
        const RESUME_FLAG = 1 << 16;
        /// Used by `iret` in hardware task switch mode to determine if current task is nested.
        const NESTED_TASK = 1 << 14;
        /// The high bit of the I/O Privilege Level field.
        ///
        /// Specifies the privilege level required for executing I/O address-space instructions.
        const IOPL_HIGH = 1 << 13;
        /// The low bit of the I/O Privilege Level field.
        ///
        /// Specifies the privilege level required for executing I/O address-space instructions.
        const IOPL_LOW = 1 << 12;
        /// Set by hardware to indicate that the sign bit of the result of the last signed integer
        /// operation differs from the source operands.
        const OVERFLOW_FLAG = 1 << 11;
        /// Determines the order in which strings are processed.
        const DIRECTION_FLAG = 1 << 10;
        /// Enable interrupts.
        const INTERRUPT_FLAG = 1 << 9;
        /// Enable single-step mode for debugging.
        const TRAP_FLAG = 1 << 8;
        /// Set by hardware if last arithmetic operation resulted in a negative value.
        const SIGN_FLAG = 1 << 7;
        /// Set by hardware if last arithmetic operation resulted in a zero value.
        const ZERO_FLAG = 1 << 6;
        /// Set by hardware if last arithmetic operation generated a carry ouf of bit 3 of the
        /// result.
        const AUXILIARY_CARRY_FLAG = 1 << 4;
        /// Set by hardware if last result has an even number of 1 bits (only for some operations).
        const PARITY_FLAG = 1 << 2;
        /// Set by hardware if last arithmetic operation generated a carry out of the
        /// most-significant bit of the result.
        const CARRY_FLAG = 1 << 0;
    }
}

impl RFlags {
    /// Returns the current value of the RFLAGS register.
    ///
    /// Drops any unknown bits.
    pub fn read() -> RFlags {
        RFlags::from_bits_truncate(Self::read_raw())
    }

    /// Returns the raw current value of the RFLAGS register.
    pub fn read_raw() -> u64 {
        let r: u64;
        unsafe {
            asm!("pushfq",
            "pop {0}",
            out(reg) r
            );
        };
        r
    }

    /// Writes the RFLAGS register, preserves reserved bits.
    pub fn write(flags: RFlags) {
        let old_value = Self::read_raw();
        let reserved = old_value & !(RFlags::all().bits());
        let new_value = reserved | flags.bits();

        Self::write_raw(new_value);
    }

    /// Writes the RFLAGS register.
    ///
    /// Does not preserve any bits, including reserved bits.
    pub fn write_raw(val: u64) {
        unsafe {
            asm!(
                "pushq {0}",
                "popf",
                in(reg) val
            );
        };
    }
}
