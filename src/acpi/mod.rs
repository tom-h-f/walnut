use crate::acpi::structures::Madt;
use crate::{efi, println};
use crate::mm::{self, PhysAddr};
use core::mem::size_of;

pub mod structures;
use structures::{Result, Error, TableType, RsdpExtended, Table};

pub unsafe fn init() -> Result<()> {

    // Get the ACPI base address from EFI
    let rsdp_addr = efi::get_acpi_table().ok_or(Error::RsdpNotFound)?;

    // Read the ACPI table at the base address
    let rsdp = RsdpExtended::from_addr(rsdp_addr)?;

    // Get XSDT
    let (xsdt, typ, 
        x_addr, len) = Table::from_addr(PhysAddr(rsdp.xsdt_address))?;

    if typ != TableType::Xsdt {
        return Err(Error::TableTypeMismatch((TableType::Xsdt, typ)));
    }

    if (len % size_of::<u64>()) != 0 {
        return Err(Error::XsdtBadEntries);
    }

    let entries = len / size_of::<u64>();

    println!("Loading {} tables from XSDT...", entries);
    for ii in 0..entries {
        // Get the physical address of the XSDT entry
        let entry_addr = ii.checked_mul(size_of::<u64>()).and_then(|f| {
            f.checked_add(x_addr.0 as usize)
        }).ok_or(Error::IntegerOverflow)?;

        // Read table pointer from the XSDT
        let table_addr = mm::readpu::<PhysAddr>( PhysAddr(entry_addr));

        // Parse and validate table header
        let (_, typ, 
            addr, len) = Table::from_addr(table_addr)?;

            match typ {
                TableType::Madt => {
                    println!("MADT Table - Length: {:<4?} | Addr: {:#016X?}", len, addr.0);
                    let madt = Madt::from_addr(addr, len)?;
                },
                _ => {},
            }

    }

    println!("{:#x?}", xsdt);

    Ok(())
}
