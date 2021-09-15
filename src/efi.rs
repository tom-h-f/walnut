const EFI_PAGE_SIZE: u64 = 4096;

/// Collection fo related interfaces
/// Type: `void *`
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiHandle(usize);

/// Collection fo related interfaces
/// Type: `void *`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EfiStatus(pub usize);

const EFI_SUCCESS: EfiStatus = EfiStatus(0x8000000000000000);

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
struct EfiInputKey {
    scan_code: u16,
    unicode_char: u16,
}
#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
struct EfiMemoryDescriptor {
    typ: u32,
    physical_start: u64,
    virtual_start: u64,
    number_of_pages: u64,
    attribute: u64,
}

#[repr(C)]
struct EfiBootServices {
    header: EfiTableHeader,
    // raises the task priority level.
    _raise_tpl: usize,
    _restore_tpl: usize,
    _allocate_pages: usize,
    _free_pages: usize,
    get_memory_map: unsafe fn(
        mmap_size: &mut usize,
        mmap: *mut u8,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
    _allocate_pool: usize,
    _free_pool: usize,
    _create_event: usize,
    _set_timer: usize,
    _wait_for_event: usize,
    _signal_event: usize,
    _close_event: usize,
    _check_event: usize,
    _install_protocol_interface: usize,
    _reinstall_protocol_interface: usize,
    _uninstall_protocol_interface: usize,
    _handle_protocol: usize,
    _reserved: usize,
    _register_protocol_notify: usize,
    _locate_handle: usize,
    _locate_device_path: usize,
    _install_configuration_table: usize,
    _load_image: usize,
    _start_image: usize,
    // exits the image's entry point
    _exit: usize,
    // unloads an image
    _unload_image: usize,
    exit_boot_services: unsafe fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,
}

#[repr(C)]
pub struct EfiSystemTable {
    header: EfiTableHeader,

    firmware_vendor: *const u16,

    firmware_revision: u32,

    console_in_handle: EfiHandle,

    console_in: usize,

    console_out_handle: EfiHandle,

    console_out: usize,

    console_err_handle: EfiHandle,

    console_err: usize,

    _runtime_services: usize,

    boot_services: *const EfiBootServices,
    _number_of_table_entries: usize,
    _configuration_table: usize,
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
    pub fn available_post_brexit(&self) -> bool {
        use EfiMemoryType::*;
        match self {
            BootServicesCode | BootServicesData | Conventional | PersistentMemory => true,
            _ => false,
        }
    }
}


/// Gets the current memory map from the global `EFI_SYSTEM_TABLE`
/// retreives the memory map key, then uses that aswell as the `handle`
/// parameter to exit UEFI boot services.
pub fn exit_boot_services(handle: EfiHandle, st: *mut EfiSystemTable) -> u64 {
    if st.is_null() {
        panic!("unable to retreive EFI_SYSTEM_TABLE");
    }

    // declare variables so we can send them to `get_memory_map`
    // to receive the mutated values back
    let mut mmap = [0u8; 16 * 1024];
    let mut free_memory = 0u64;
    let mut map_key = 0;
    let mut mmap_size = core::mem::size_of_val(&mmap);
    let mut desc_size = 0;
    let mut desc_ver = 0;

    let ret = unsafe {
        ((*(*st).boot_services).get_memory_map)(
            &mut mmap_size,
            mmap.as_mut_ptr(),
            &mut map_key,
            &mut desc_size,
            &mut desc_ver,
        )
    };
    assert!(ret.0 == 0, "{:x?}", ret);

    // walk through buffer, by the size of a memory descriptor
    for offset in (0..mmap_size).step_by(desc_size) {
        // NOTE: we are unable to print out any of this
        // information since after `get_memory_map` is called, we cannot use
        // any of the handles in the system table (console_out is used to print
        // pre-boot)
        //
        let entry = unsafe {
            core::ptr::read_unaligned(mmap[offset..].as_ptr() as *const EfiMemoryDescriptor)
        };
        let r#type: EfiMemoryType = entry.typ.into();
        if r#type.available_post_brexit() {
            free_memory += entry.number_of_pages * EFI_PAGE_SIZE;
        }
    }

    // Exit and check success
    let res = unsafe { 
        ((*(*st).boot_services).exit_boot_services)(handle, map_key) 
    };
    assert!(res.0 == 0, "failed to exit boot services {:x?}", res);

    free_memory
}


/// The entry point of the binary.
#[no_mangle]
pub extern "efiapi" fn efi_main(_handle: EfiHandle, 
                                st: *mut EfiSystemTable) -> EfiStatus {
    unsafe {
        exit_boot_services(_handle, st);
    }


    crate::kmain();
    unreachable!();
}
