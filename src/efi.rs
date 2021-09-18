use crate::mm::PhysAddr;
use crate::mm::rangeset::{Range, RangeSet};

#[allow(dead_code)]
const EFI_PAGE_SIZE: u64 = 4096;
#[allow(dead_code)]
const EFI_SUCCESS: EfiStatus = EfiStatus(0x8000000000000000);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct EfiGuid(u32, u16, u16, [u8; 8]);

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
struct EfiSimpleTextInputProtocol {
    /// Resets the input device hardware.
    reset: unsafe fn(
        this: *const EfiSimpleTextInputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    /// Reads the next keystroke from the input device.
    read_keystroke:
        unsafe fn(this: *const EfiSimpleTextInputProtocol, key: *mut EfiInputKey) -> EfiStatus,
    /// Event to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for a key to
    /// be available
    _wait_for_key: usize,
}
#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    /// Resets the output device hardware.
    reset: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    /// Writes a string to the device.
    output_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    /// Verifies that all chars in a string can be output
    /// to the target device.
    test_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    /// Returns information for an available text mode that the output
    /// device(s) supports.
    _query_mode: usize,

    /// Sets the output device(s) to a specified mode.
    _set_mode: usize,

    _set_attribute: usize,

    _clear_screen: usize,

    _set_cursor_position: usize,

    _enable_cursor: usize,

    _mode: usize,
}

#[repr(C)]
pub struct EfiSystemTable {
    header: EfiTableHeader,

    firmware_vendor: *const u16,

    firmware_revision: u32,

    console_in_handle: EfiHandle,

    console_in: *const EfiSimpleTextInputProtocol,

    console_out_handle: EfiHandle,

    console_out: *const EfiSimpleTextOutputProtocol,

    console_err_handle: EfiHandle,

    console_err: *const EfiSimpleTextOutputProtocol,

    _runtime_services: usize,

    boot_services: *const EfiBootServices,
    _number_of_tables: usize,
    tables: *const EfiConfigurationTable,
}

#[derive(Debug)]
#[repr(C)]
pub struct EfiConfigurationTable {
    guid: EfiGuid,
    /// a pointer ot the `VendorTable`
    table: usize,
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
    pub fn available_post_brexit(&self) -> bool {
        use EfiMemoryType::*;
        match self {
            BootServicesCode | BootServicesData | Conventional | PersistentMemory => true,
            _ => false,
        }
    }
}

#[macro_use]
pub mod print {
    use core::fmt::{Result, Write};

    pub struct ScreenWriter;

    impl Write for ScreenWriter {
        fn write_str(&mut self, string: &str) -> Result {
            crate::efi::output_string(string);
            Ok(())
        }
    }

    pub fn _print(args: core::fmt::Arguments) {
        <ScreenWriter as core::fmt::Write>::write_fmt(&mut ScreenWriter, args).unwrap();
    }

    /// The standard Rust `print!()` macro
    #[macro_export]
    macro_rules! efi_print {
    ($($arg:tt)*) => ($crate::efi::print::_print(format_args!($($arg)*)));
}

    #[macro_export]
    macro_rules! efi_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::efi_print!("{}\n", format_args!($($arg)*)));
}
}
use core::sync::atomic::{AtomicPtr, Ordering};

/// Global EFI system table which is saved upon entry of the kernel.
static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());

pub unsafe fn register_system_table(system_table: *mut EfiSystemTable) {
    EFI_SYSTEM_TABLE
        .compare_exchange(
            core::ptr::null_mut(),
            system_table,
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .unwrap();
}

pub fn output_string(string: &str) {
    // get the system table
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    if st.is_null() {
        return;
    }

    let out = unsafe { (*st).console_out };

    let mut tmp = [0u16; 32];
    let mut in_use = 0;

    // note that UEFI uses UCS-2 not UTF-16 so don't have to worry about
    // 32-bit code points
    for c in string.encode_utf16() {
        // inject CR if needed. We always make sure there's room
        // for one based on the way we check the buffer len (-2 instead of -1)
        if c == b'\n' as u16 {
            tmp[in_use] = b'\r' as u16;
            in_use += 1;
        }

        // write a char into the buffer
        tmp[in_use] = c;
        in_use += 1;

        if in_use == (tmp.len() - 2) {
            // null terminate the buffer
            tmp[in_use] = 0;

            unsafe {
                ((*out).output_string)(out, tmp.as_ptr());
            }

            in_use = 0;
        }
    }

    if in_use > 0 {
        tmp[in_use] = 0;
        unsafe {
            ((*out).output_string)(out, tmp.as_ptr());
        }
    }
}

/// Gets the current memory map from the global `EFI_SYSTEM_TABLE`
/// retreives the memory map key, then uses that aswell as the `handle`
/// parameter to exit UEFI boot services.
pub fn exit_boot_services(handle: EfiHandle) -> RangeSet {

    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
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
    // After we call get_memory_map we cannot perform any prints or the map_key will 
    // cause the attempted exit to return `EFI_INVALID_PARAMETER`


    let mut memory_ranges = RangeSet::new();

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
        let typ: EfiMemoryType = entry.typ.into();
        // If memory type will be available after we exit UEFI boot services
        // add to memory ranges.
        if typ.available_post_brexit() {
            memory_ranges.insert(Range { 
                start: entry.physical_start,
                end: entry.physical_start + (entry.number_of_pages * EFI_PAGE_SIZE)
            });
        };

    }

    // Exit and check success
    let res = unsafe { ((*(*st).boot_services).exit_boot_services)(handle, map_key) };

    assert!(res.0 == 0, "failed to exit boot services {:x?}", res);
    // destroy the EFI system table
    EFI_SYSTEM_TABLE.store(core::ptr::null_mut(), Ordering::SeqCst);

    memory_ranges
}

use crate::acpi;
/// The entry point of the binary.
#[no_mangle]
pub extern "efiapi" fn efi_main(_handle: EfiHandle, st: *mut EfiSystemTable) -> EfiStatus {
    unsafe {
        register_system_table(st);
        acpi::init().expect("Failed to initalize ACPI");
        let memory_map = exit_boot_services(_handle);
        efi_println!("{:#X?}", memory_map);
    }

    crate::kmain();
    unreachable!();
}



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

    let tables = unsafe { core::slice::from_raw_parts((*st).tables, (*st)._number_of_tables) };

    // Attempt to find the table with the ACPI 2.0 GUID, if can't find
    // attempt to find table with ACPI 1.0 GUID.
    if let Some(acpi) = tables
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
    {
        efi_println!("ACPI Table at {:#x?} {:#x?}", acpi, unsafe {
            core::ptr::read_unaligned(acpi as *const u64)
        });
        Some(PhysAddr(acpi))
    } else {
        None
    }
}
