use super::apic::*;
use super::{Error, Result, TableType};
use crate::acpi::structures::apic::ApicRecordType;
use crate::efi_println;
use crate::mm::{self, PhysAddr};
use core::mem::size_of;

#[repr(C, packed)]
pub struct Madt {
    local_apic_addr: u32,
    flags: u32, // TODO add bitflags
}
impl Madt {
    pub unsafe fn from_addr(addr: PhysAddr, size: usize) -> Result<Self> {
        const E: Error = Error::LengthMismatch(TableType::Madt);

        let mut slice = mm::PhysSlice::new(addr, size);

        let local_apic_addr = slice.consume::<u32>().map_err(|_| E)?;

        // Get APIC flags
        let flags = slice.consume::<u32>().map_err(|_| E)?;
        let ret = Self {
            local_apic_addr,
            flags,
        };

        // Handle Interrupt Contoller Structures
        while slice.len() > 0 {
            // Read the interrupt controller structure header
            let typ = slice.consume::<u8>().map_err(|_| E)?;
            let len = slice
                .consume::<u8>()
                .map_err(|_| E)?
                .checked_sub(2)
                .ok_or(E)?;

            efi_println!("{:#x} {}", typ, len);

            // Could probably be done better with
            // a generic and bindings for size functions?
            // ah well!
            match ApicRecordType::from(typ) {
                ApicRecordType::ProcessorLocalApic => {
                    // Ensure data is correct size
                    if len as usize != size_of::<ProcessorLocalApic>() {
                        return Err(E);
                    }

                    let apic = slice.consume::<ProcessorLocalApic>().map_err(|_| E)?;
                    efi_println!("{:#x?}", apic);
                }
                ApicRecordType::IoApic => {
                    // Ensure data is correct size
                    if len as usize != size_of::<IoApic>() {
                        return Err(E);
                    }

                    let apic = slice.consume::<IoApic>().map_err(|_| E)?;
                    efi_println!("{:#x?}", apic);
                }
                ApicRecordType::IoApicInterruptSourceOverride => {
                    // Ensure data is correct size
                    if len as usize != size_of::<IoApicInterruptSourceOverride>() {
                        return Err(E);
                    }

                    let apic = slice
                        .consume::<IoApicInterruptSourceOverride>()
                        .map_err(|_| E)?;
                    efi_println!("{:#x?}", apic);
                }
                ApicRecordType::IoApicNonMaskableInterruptSource => {
                    // Ensure data is correct size
                    if len as usize != size_of::<IoApicNonMaskableInterruptSource>() {
                        return Err(E);
                    }

                    let apic = slice.consume::<ProcessorLocalApic>().map_err(|_| E)?;
                    efi_println!("{:#x?}", apic);
                }
                ApicRecordType::LocalApicNonMaskableInterrupts => {
                    // Ensure data is correct size
                    if len as usize != size_of::<LocalApicNonMaskableInterrupts>() {
                        return Err(E);
                    }

                    let apic = slice
                        .consume::<LocalApicNonMaskableInterrupts>()
                        .map_err(|_| E)?;
                    efi_println!("{:#x?}", apic);
                }
                ApicRecordType::LocalApicAddressOverride => {
                    // Ensure data is correct size
                    if len as usize != size_of::<LocalApicAddressOverride>() {
                        return Err(E);
                    }

                    let apic = slice.consume::<LocalApicAddressOverride>().map_err(|_| E)?;
                    efi_println!("{:#x?}", apic);
                }
                ApicRecordType::ProcessorLocalX2Apic => {
                    // Ensure data is correct size
                    if len as usize != size_of::<LocalX2Apic>() {
                        return Err(E);
                    }

                    let apic = slice.consume::<LocalX2Apic>().map_err(|_| E)?;
                    efi_println!("{:#x?}", apic);
                }
                _ => {
                    slice.discard(len as usize).map_err(|_| E)?;
                }
            }
        }

        efi_println!("{:#x}", local_apic_addr);

        Ok(ret)
    }
}
