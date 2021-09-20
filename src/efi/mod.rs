//! Module defining various items to boot to our OS correctly from UEFI.
use crate::memory::{MemoryRange, PhysAddr};
use core::sync::atomic::Ordering;


pub mod structures;
use structures::*;
pub mod acpi;


fn get_memory_map(st: &EfiSystemTable) -> Result<(MemoryRange, u64)> {
    // Declare variables so we can send them to `get_memory_map`
    // to receive the mutated values back
    let mut memory_map = [0u8; 8 * 1024];
    let mut key = 0;
    let mut mmap_size = core::mem::size_of_val(&memory_map) as u64;
    let mut desc_size = 0;
    let mut desc_ver = 0;

    let ret = unsafe { 
             ((*st.boot_services).get_memory_map)(&mut mmap_size, &mut memory_map as *mut u8,                                                 &mut key, &mut desc_size, &mut desc_ver)
    };
    if ret.0 != 0  {
        return Err(Error::CouldntGetMemoryMap(ret));
    }  

    let mut mr = MemoryRange::new();

    // Walk through the buffer, and find the largest region
    // store this information and return it.
    for offset in (0..mmap_size).step_by(desc_size as usize) {
        // NOTE: we are unable to print out any of this
        // information since after `get_memory_map` is called, we cannot use
        // any of the handles in the system table (console_out is used to print
        // pre-boot)
        //
        let entry = unsafe {
            core::ptr::read_unaligned(memory_map[offset as usize..].as_ptr() as *const EfiMemoryDescriptor)
        };
        let r#type: EfiMemoryType = entry.typ.into();
        if r#type.available_post_exit_boot_services() && 
        ((entry.physical_start + (entry.number_of_pages * EFI_PAGE_SIZE)) - entry.physical_start) > mr.size()  {
            mr.start = entry.physical_start;
            mr.end = entry.physical_start + (entry.number_of_pages * EFI_PAGE_SIZE);
        }
    }

    // Check we actually found a valid memory area
    if mr.start == 0 || mr.end == 0 {
        return Err(Error::NoValidMemoryArea)
    }

    Ok((mr, key))


}

/// Initializes the system table, must be called before exit.
pub unsafe fn init(st: &mut EfiSystemTable) -> Result<()> {
    register_system_table(st)?;
    Ok(())
}

/// Gets the current memory map from the global `EFI_SYSTEM_TABLE`
/// retreives the memory map key, then uses that aswell as the `handle`
/// parameter to exit UEFI boot services.
pub unsafe fn exit_boot_services(handle: EfiHandle) -> Result<MemoryRange> {
    let st = load_system_table()?;
    
    // After we call get_memory_map we cannot perform any prints or the map_key will
    // cause the attempted exit to return `EFI_INVALID_PARAMETER`
    let (mr, key) = get_memory_map(st)?;

    // Perform the exit
    exit_boot_service_int(st, handle, key)?;
  
    Ok(mr)
}
