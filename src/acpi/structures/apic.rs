
/// We should get one of these for each core
#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct LocalApic {
    acpi_processor_uid: u8,
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
pub struct LocalX2Apic {
    /// Must be zero
    reserved: u16,
    /// processor's local x2APIC ID
    x2apic_id: u32,
    /// same as Local APIC Flags
    flags: LocalApicFlags,

    acpi_proc_id: u32,
}