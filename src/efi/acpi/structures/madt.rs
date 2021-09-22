use super::apic::*;
use super::{Error, Result, TableType};
use crate::efi::acpi::structures::apic::ApicRecordType;
use crate::memory::{self, PhysAddr};
use core::mem::size_of;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct Madt {
    local_apic_addr: u32,
    flags: u32, // TODO add bitflags
}
impl Madt {
    pub unsafe fn from_addr(addr: PhysAddr, size: u64) -> Result<Self> {
        const E: Error = Error::LengthMismatch(TableType::Madt);

        let mut slice = crate::memory::PhysSlice::new(addr, size as usize);

        let local_apic_addr = slice.consume::<u32>().map_err(|_| E)?;

        // Get APIC flags
        let flags = slice.consume::<u32>().map_err(|_| E)?;
        let ret = Self {
            local_apic_addr,
            flags,
        };

        let mut total_procs = 0;
        let mut io_apic_addr = 0;
        // Handle Interrupt Contoller Structures
        while !slice.is_empty() {
            // Read the interrupt controller structure header
            let typ = slice.consume::<u8>().map_err(|_| E)?;
            let len = slice
                .consume::<u8>()
                .map_err(|_| E)?
                .checked_sub(2)
                .ok_or(E)? as usize;

            // Could probably be done better with
            // a generic and bindings for size functions?
            // ah well!
            match ApicRecordType::from(typ) {
                ApicRecordType::ProcessorLocalApic => {
                    // Ensure data is correct size
                    if len != size_of::<ProcessorLocalApic>() {
                        return Err(E);
                    }

                    let _apic = slice.consume::<ProcessorLocalApic>().map_err(|_| E)?;
                    total_procs += 1;
                    // TODO should probably check if each core that comes through here is `ENABLED` and if not do something!
                }
                ApicRecordType::IoApic => {
                    // Ensure data is correct size
                    if len != size_of::<IoApic>() {
                        return Err(E);
                    }

                    let _apic = slice.consume::<IoApic>().map_err(|_| E)?;
                    io_apic_addr = _apic.io_apic_addr;
                }
                ApicRecordType::IoApicInterruptSourceOverride => {
                    // Ensure data is correct size
                    if len != size_of::<IoApicInterruptSourceOverride>() {
                        return Err(E);
                    }

                    let _apic = slice
                        .consume::<IoApicInterruptSourceOverride>()
                        .map_err(|_| E)?;
                }
                ApicRecordType::IoApicNonMaskableInterruptSource => {
                    // Ensure data is correct size
                    if len != size_of::<IoApicNonMaskableInterruptSource>() {
                        return Err(E);
                    }

                    let _apic = slice.consume::<ProcessorLocalApic>().map_err(|_| E)?;
                }
                ApicRecordType::LocalApicNonMaskableInterrupts => {
                    // Ensure data is correct size
                    if len as usize != size_of::<LocalApicNonMaskableInterrupts>() {
                        return Err(E);
                    }

                    let _apic = slice
                        .consume::<LocalApicNonMaskableInterrupts>()
                        .map_err(|_| E)?;
                }
                ApicRecordType::LocalApicAddressOverride => {
                    // Ensure data is correct size
                    if len != size_of::<LocalApicAddressOverride>() {
                        return Err(E);
                    }

                    let _apic = slice.consume::<LocalApicAddressOverride>().map_err(|_| E)?;
                }
                ApicRecordType::ProcessorLocalX2Apic => {
                    // Ensure data is correct size
                    if len != size_of::<LocalX2Apic>() {
                        return Err(E);
                    }

                    let _apic = slice.consume::<LocalX2Apic>().map_err(|_| E)?;
                }
                ApicRecordType::Unknown(_) => {
                    slice.discard(len as u64).map_err(|_| E)?;
                }
            }
        }

        crate::println!(
            "Found {} cores, IOAPIC: {:#X?}, LAPIC: {:#X?}",
            total_procs,
            local_apic_addr,
            io_apic_addr
        );

        Ok(ret)
    }
}
