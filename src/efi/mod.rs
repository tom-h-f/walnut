//! Module defining various items to boot to our OS correctly from UEFI.
use crate::memory::{MemoryRange, PhysAddr};
use core::sync::atomic::Ordering;


pub mod structures;
use structures::*;
pub mod acpi;
pub mod memory;



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
    let (mr, key) = memory::get_memory_map(st)?;

    // Perform the exit
    exit_boot_service_int(st, handle, key)?;
  
    Ok(mr)
}
