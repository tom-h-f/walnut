use super::apic::{LocalApic, LocalX2Apic};
use super::{Table, TableType, Error, Result};
use crate::mm::{self, PhysAddr};
use crate::println;
use core::mem::size_of;


#[repr(C, packed)]
pub struct Madt {
    table: Table,
}
impl Madt {
    pub unsafe fn from_addr(addr: PhysAddr, size: usize) -> Result<Self> {
        const E: Error = Error::LengthMismatch(TableType::Madt);

        let mut slice = mm::PhysSlice::new(addr, size);

        let local_apic_addr = slice.consume::<u32>().map_err(|_| E)?;

        // Get APIC flags
        let local_apic_flags = slice.consume::<u32>().map_err(|_| E)?;

        // Handle Interrupt Contoller Structures
        while slice.len() > 0 {
            // Read the interrupt controller structure header
            let typ = slice.consume::<u8>().map_err(|_| E)?;
            let len = slice
                .consume::<u8>()
                .map_err(|_| E)?
                .checked_sub(2)
                .ok_or(E)?;

            println!("{:#x} {}", typ, len);

            match typ {
                0 => {
                    // Ensure data is correct size
                    if len as usize != size_of::<LocalApic>() {
                        return Err(E);
                    }

                    let apic = slice.consume::<LocalApic>().map_err(|_| E)?;
                    println!("{:#x?}", apic);
                }
                9 => {
                    
                    // Ensure data is correct size
                    if len as usize != size_of::<LocalX2Apic>() {
                        return Err(E);
                    }

                    let apic = slice.consume::<LocalX2Apic>().map_err(|_| E)?;
                    println!("{:#x?}", apic);
                }
                _ => {
                    slice.discard(len as usize).map_err(|_| E)?;
                }
            }
        }

        println!("{:#x}", local_apic_addr);

        panic!();
    }
}