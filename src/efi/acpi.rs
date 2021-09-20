use super::*;

pub fn get_acpi_table() -> Option<PhysAddr> {
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
        .map(|a|PhysAddr(a as u64))
} 
