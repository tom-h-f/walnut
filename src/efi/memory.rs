use super::{EfiSystemTable, Error, Result, EFI_PAGE_SIZE};
use crate::{memory::{Range, RangeSet}, print};

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub typ: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum EfiMemoryType {
    /// This enum variant is not used.
    Reserved = 0,
    /// The code portions of a loaded UEFI application.
    LoaderCode = 1,
    /// The data portions of a loaded UEFI applications,
    /// as well as any memory allocated by it.
    #[allow(dead_code)]
    LoaderData = 2,
    /// Code of the boot drivers.
    ///
    /// Can be reused after OS is loaded.
    BootServicesCode = 3,
    /// Memory used to store boot drivers' data.
    ///
    /// Can be reused after OS is loaded.
    BootServicesData = 4,
    /// Runtime drivers' code.
    RuntimeServicesCode = 5,
    /// Runtime services' code.
    RuntimeServicesData = 6,
    /// Free usable memory.
    Conventional = 7,
    /// Memory in which errors have been detected.
    Unusable = 8,
    /// Memory that holds ACPI tables.
    /// Can be reclaimed after they are parsed.
    AcpiReclaim = 9,
    /// Firmware-reserved addresses.
    AcpiNonVolatile = 10,
    /// A region used for memory-mapped I/O.
    Mmio = 11,
    /// Address space used for memory-mapped port I/O.
    MmioPortSpace = 12,
    /// Address space which is part of the processor.
    PalCode = 13,
    /// Memory region which is usable and is also non-volatile.
    PersistentMemory = 14,
}

impl From<u32> for EfiMemoryType {
    fn from(v: u32) -> Self {
        use EfiMemoryType::*;
        match v {
            0 => Reserved,
            1 => LoaderCode,
            2 => LoaderCode,
            3 => BootServicesCode,
            4 => BootServicesData,
            5 => RuntimeServicesCode,
            6 => RuntimeServicesData,
            7 => Conventional,
            8 => Unusable,
            9 => AcpiReclaim,
            10 => AcpiNonVolatile,
            11 => Mmio,
            12 => MmioPortSpace,
            13 => PalCode,
            14 => PersistentMemory,
            _ => {
                panic!("Unsupported memory type supplied!")
            }
        }
    }
}

impl EfiMemoryType {
    /// Returns whether or not the memory type is available
    /// for general purpose use after boot services have been exited (brexit).
    pub fn available_post_exit_boot_services(&self) -> bool {
        use EfiMemoryType::*;
        matches!(
            self,
            BootServicesCode | BootServicesData | Conventional | PersistentMemory
        )
    }
}

pub fn get_memory_map(st: &EfiSystemTable) -> Result<(RangeSet, u64)> {

    // Declare variables so we can send them to `get_memory_map`
    // to receive the mutated values back
    let mut memory_map = [0u8; 8 * 1024];
    let mut key = 0;
    let mut mmap_size = core::mem::size_of_val(&memory_map) as u64;
    let mut desc_size = 0;
    let mut desc_ver = 0;

    let ret = unsafe {
        ((*st.boot_services).get_memory_map)(
            &mut mmap_size,
            &mut memory_map as *mut u8,
            &mut key,
            &mut desc_size,
            &mut desc_ver,
        )
    };
    if ret.0 != 0 {
        return Err(Error::CouldntGetMemoryMap(ret));
    }

    let mut rs = RangeSet::new();

    // Walk through the buffer, and find the largest region
    // store this information and return it.
    for offset in (0..mmap_size).step_by(desc_size as usize) {
        // NOTE: we are unable to print out any of this
        // information since after `get_memory_map` is called, we cannot use
        // any of the handles in the system table (console_out is used to print
        // pre-boot)

        let entry = unsafe {
            core::ptr::read_unaligned(
                memory_map[offset as usize..].as_ptr() as *const EfiMemoryDescriptor
            )
        };
        let r#type: EfiMemoryType = entry.typ.into();
        if r#type.available_post_exit_boot_services() {
            let start = entry.physical_start;
            let end = entry.physical_start + (entry.number_of_pages * EFI_PAGE_SIZE);
            rs.insert(Range { start, end, descriptor: entry });
        }
    }

    crate::whereami!();
    // Check we actually found a valid memory area
    if !rs.any_valid() {
        return Err(Error::NoValidMemoryArea);
    }

    Ok((rs, key))
}

/// Identity maps all memory at `offset` and then informs UEFI
/// of this mapping, enabling it.
pub fn set_memory_map(st: &EfiSystemTable, range_set: &mut RangeSet, offset: u64) -> Result<()> {

    range_set.id_map(offset);

    // Create a max size array, and then get the amount of 
    // descriptors in use, and use that.
    let mut fullmap = [EfiMemoryDescriptor::default(); 256];
    let cnt = range_set.to_descriptors(&mut fullmap);
    let map = &fullmap[..cnt];

    for i in map {
        crate::println!("{:#X?} -> {:#X?}", i.physical_start, i.virtual_start);
    }

    // Setup arguments for SetVirtualAddressMap 
    let map_size = core::mem::size_of_val(map);
    let desc_ver = 1;
    let desc_size = core::mem::size_of::<EfiMemoryDescriptor>();

    if desc_size * map.len() != map_size {
        return Err(Error::MemoryMapInvalidSize)
    }

    unsafe {
        let ret = ((*st.runtime_services).set_virtual_address_map)(map_size, desc_size, desc_ver, 
                                                                map as *const [EfiMemoryDescriptor]);
        if ret.0 != 0 {
            return Err(Error::CouldntSetVirtualAddressMap(ret));
        }
    }

    Ok(())
}
