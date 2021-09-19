#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ApicRecordType {
    ProcessorLocalApic               = 0,
    IoApic                           = 1,
    IoApicInterruptSourceOverride    = 2,
    IoApicNonMaskableInterruptSource = 3,
    LocalApicNonMaskableInterrupts   = 4,
    LocalApicAddressOverride         = 5,
    ProcessorLocalX2Apic             = 9,
    Unknown(u8),
}

impl From<u8> for ApicRecordType {
    fn from(v: u8) -> Self {
        match &v {
            0 => Self::ProcessorLocalApic,
            1 => Self::IoApic,
            2 => Self::IoApicInterruptSourceOverride,
            3 => Self::IoApicNonMaskableInterruptSource,
            4 => Self::LocalApicNonMaskableInterrupts,
            5 => Self::LocalApicAddressOverride,
            9 => Self::ProcessorLocalX2Apic,
            _ => Self::Unknown(v),
        }
    }
}


bitflags! {
    struct OtherApicFlags: u16 {
        /// If set then the interrupt is active when low
        const ACTIVE_WHEN_LOW = 1 << 1;
        /// If set then interrupt is level-triggered
        const LEVEL_TRIGGERED_INTERRUPT = 1 << 7;
    }
}


/// We should get one of these for each core
/// This type represents a I/O APIC. 
/// The global system interrupt base is the first interrupt 
/// number that this I/O APIC handles. 
/// You can see how many interrupts it handles using 
/// the register by getting the number of redirection 
/// entries from register 0x01
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct ProcessorLocalApic {
    acpi_proc_id: u8,
    apic_id: u8,
    flags: LocalApicFlags,
}

use bitflags::bitflags;
bitflags! {
    struct LocalApicFlags: u32 {
        const ENABLED       = 1 << 0;
        const ONLINE_CAPABLE = 1 << 1;
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct IoApic {
    io_apic_id: u8,
    reserved: u8,
    io_apic_addr: u32,
    global_system_interrupt_base: u32,
}

/// This entry type contains the data for an Interrupt Source Override. This explains how IRQ sources are mapped to global system interrupts. 
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct IoApicInterruptSourceOverride {
    bus_source: u8,
    irq_source: u8,
    global_system_interrupt: u32,
    flags: OtherApicFlags,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct IoApicNonMaskableInterruptSource {
    nmi_source: u8,
    reserved: u8,
    flags: OtherApicFlags,
    global_system_interrupt: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct LocalApicNonMaskableInterrupts {
    /// ACPI Processor ID (0xFF means all processors)
    acpi_proc_id: u8,
    flags: OtherApicFlags,
    /// Should be configured with the `LINT0` and `LINT1` entries in the 
    /// local vector table of the relevant processors local APIC.
    lint: u8,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct LocalApicAddressOverride {
    /// ACPI Processor ID (0xFF means all processors)
    reserved: u16,
    /// 64-bit physical address of Local APIC
    local_apic_physaddr: u64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct LocalX2Apic {
    /// Must be zero
    reserved: u16,
    /// processor's local x2APIC ID
    x2apic_id: u32,
    /// same as Local APIC Flags
    flags: LocalApicFlags,
    acpi_id: u32,
}