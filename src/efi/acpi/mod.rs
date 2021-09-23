use super::*;
use core::mem::size_of;

pub mod structures;
use structures::{Madt, Rsdp, RsdpExtended, Table, TableType};

unsafe fn get_acpi_table() -> Option<PhysAddr> {
    // ACPI 2.0 GUID
    const EFI_ACPI_TABLE_GUID: EfiGuid = EfiGuid(
        0x8868e871,
        0xe4f1,
        0x11d3,
        [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
    );
    // ACPI 1.0 GUID
    const ACPI_TABLE_GUID: EfiGuid = EfiGuid(
        0xeb9d2d30,
        0x2d88,
        0x11d3,
        [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
    );

    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
    if st.is_null() {
        panic!("unable to retreive EFI_SYSTEM_TABLE");
    }

    let tables = unsafe { core::slice::from_raw_parts((*st).tables, (*st).number_of_tables) };

    // Attempt to find the table with the ACPI 2.0 GUID, if can't find
    // attempt to find table with ACPI 1.0 GUID.
    tables
        .iter()
        .find_map(|EfiConfigurationTable { guid, table }| {
            (guid == &EFI_ACPI_TABLE_GUID).then_some(*table)
        })
        .or_else(|| {
            tables
                .iter()
                .find_map(|EfiConfigurationTable { guid, table }| {
                    (guid == &ACPI_TABLE_GUID).then_some(*table)
                })
        })
        .map(|a| PhysAddr(a as u64))
}


pub unsafe fn init() -> Result<()> {
    // Get the ACPI base address from EFI
    let rsdp_addr = get_acpi_table().ok_or(Error::RsdpNotFound)?;

    // Read the ACPI table at the base address
    let rsdp = RsdpExtended::from_addr(rsdp_addr)?;

    crate::println!("{:#x?}", rsdp);

    // Get XSDT
    let (_xsdt, typ, x_addr, len) = Table::from_addr(PhysAddr(rsdp.xsdt_address))?;

    if typ != TableType::Xsdt {
        return Err(Error::TableTypeMismatch((TableType::Xsdt, typ)));
    }

    if (len % size_of::<u64>() as u64) != 0 {
        return Err(Error::XsdtBadEntries);
    }

    crate::println!("{:#x?}", _xsdt);

    let entries = len / size_of::<u64>() as u64;

    for ii in 0..entries {
        // Get the physical address of the XSDT entry
        let entry_addr = ii
            .checked_mul(size_of::<u64>() as u64)
            .and_then(|f| f.checked_add(x_addr.0))
            .ok_or(Error::IntegerOverflow)?;

        // Read table pointer from the XSDT
        let table_addr = crate::memory::readpu::<PhysAddr, PhysAddr>(PhysAddr(entry_addr));

        // Parse and validate table header
        let (_, typ, addr, len) = Table::from_addr(table_addr)?;

        match typ {
            TableType::Madt => {
                let _madt = Madt::from_addr(addr, len)?;
                crate::println!("{:#x?}", _madt);
            },
            TableType::Rsdp => {
                let _rsdp = Rsdp::from_addr(addr)?;
                crate::println!("{:#x?}", _rsdp);
            },
            TableType::RsdpExtended => {
                let _rsdp = RsdpExtended::from_addr(addr)?;
                crate::println!("{:#x?}", _rsdp);
            },
            _ => {},
        }
    }

    Ok(())
}