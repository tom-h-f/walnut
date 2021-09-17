use crate::efi;
use crate::mm::{self, PhysAddr};
use crate::println;
use core::mem::size_of;

// A `Result` type that wraps an ACPI error
type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum Error {
    /// The ACPI table address was not reported by UEFI
    RsdpNotFound,

    /// An ACPI table had an invalid checksum
    ChecksumMismatch(TableType),

    /// An ACPI table did not have the correct signature
    SignatureMismatch(TableType),

    /// Attemped to access Extended Rsdp (ACPI >= 2.0) but we are in ACPI 1.0
    RevisionTooOld,

    /// The Rsdp Extended Size received is incorrect.
    RsdpExtendedSizeMisMatch,

    /// The table has an incorrect or unexpected type
    /// this error should hold the expected, and then the found value
    TableTypeMismatch((TableType, TableType)),
}

/// Different types of ACPI tables
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableType {
    /// Root System Description Pointer
    Rsdp,

    /// Root System Description Pointer Extended (ACPI 2.0 only)
    RsdpExtended,

    // System Descriptor Table 
    Table,

    // Extended System Description Table
    Xsdt,

    // An unknown Table Type
    Unknown([u8; 4]),
}

impl From<[u8; 4]> for TableType {
    fn from(v: [u8; 4]) -> Self {
        match &v {
            b"XSDT" => Self::Xsdt,
            b"RSDP" => Self::Xsdt,
            _ => Self::Unknown(v),
        }
    }
}




#[repr(C, packed)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
}

use core::fmt;
impl fmt::Debug for Rsdp {
    #[allow(unaligned_references)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RSDP")
            .field("signature", &core::str::from_utf8(&self.signature).unwrap())
            .field("oem_id", &core::str::from_utf8(&self.oem_id).unwrap())
            .field("revision", &self.revision)
            .field("rsdt_address", unsafe {
                &core::ptr::read_unaligned(&self.rsdt_address)
            })
            .finish()
    }
}

impl Rsdp {
    unsafe fn from_addr(addr: PhysAddr) -> Result<Self> {
        let rsdp = mm::read_physaddr::<Rsdp>(addr);

        compute_checksum(addr, size_of::<Self>(), TableType::Rsdp)?;

        rsdp.check_signature()?;

        Ok(rsdp)
    }

    fn check_signature(&self) -> Result<()> {
        if &self.signature != b"RSD PTR " {
            return Err(Error::SignatureMismatch(TableType::Rsdp));
        }
        Ok(())
    }

    /// Checks the revision of this ACPI table.
    /// If below 2.0, we Error
    fn check_revision(&self) -> Result<()> {
        if self.revision < 2 {
            return Err(Error::RevisionTooOld);
        }
        Ok(())
    }
}
#[repr(C, packed)]
pub struct RsdpExtended {
    descriptor: Rsdp,
    length: u32,
    xsdt_address: usize,
    extended_checksum: u8,
    _reserved: [u8; 3],
}

impl RsdpExtended {
    pub unsafe fn from_addr(addr: PhysAddr) -> Result<Self> {
        // Read the RSDP
        // This is the structure compat with ACPI 1.0 and ACPI 2.0
        let rsdp = Rsdp::from_addr(addr)?;

        // We only support ACPI >= 2.0
        rsdp.check_revision()?;

        // Read the Extended RSDP
        let rsdp = mm::read_physaddr::<RsdpExtended>(addr);

        rsdp.check_length()?;

        compute_checksum(addr, size_of::<Self>(), TableType::RsdpExtended)?;

        Ok(rsdp)
    }

    fn check_length(&self) -> Result<()> {
        if self.length as usize != size_of::<RsdpExtended>() {
            return Err(Error::RsdpExtendedSizeMisMatch)
        }
        Ok(())
    }
}

impl fmt::Debug for RsdpExtended {
    #[allow(unaligned_references)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RSDP (for APIC >= 2.0)")
            .field("", &self.descriptor)
            .field("length", &self.length)
            .field("xsdt_address", unsafe {
                &core::ptr::read_unaligned(&self.xsdt_address)
            })
            .field("extended_checksum", &self.extended_checksum)
            .finish()
    }
}

/// System Descriptor Table Header
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Table {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    _reserved: [u8; 3],
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl fmt::Debug for Table {
    #[allow(unaligned_references)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("APIC Table")
            .field("Signature", &core::str::from_utf8(&self.signature).unwrap())
            .field("Length", &self.length)
            .field("Revision", &self.revision)
            .field("OEM ID", &core::str::from_utf8(&self.oem_id).unwrap())
            .field("OEM Table ID", &core::str::from_utf8(&self.oem_table_id).unwrap())
            // I don't know if creator ID is actually a string but its more fun this way
            .field("Creator ID", &core::str::from_utf8(&self.creator_id.to_le_bytes()).unwrap()) 
            .field("Creator Revision", &self.creator_revision)
            .finish()
    }
}

impl Table {
    unsafe fn from_addr(addr: PhysAddr, req_type: TableType) -> Result<Self> {
        // Read the table
        let table = mm::read_physaddr::<Self>(addr);

        // Get the type of this table
        let r#type = TableType::from(table.signature);

        compute_checksum(addr, table.length as usize, r#type)?;

        if r#type == req_type {
            Ok(table)
        } else {
            Err(Error::TableTypeMismatch((req_type, r#type)))
        }
    }
}
#[repr(C, packed)]
pub struct Rsdt {
    table: Table,
    /// this points to Other SDT
    other_sdt_ptr: usize,
}

pub unsafe fn init() -> Result<()> {

    // Get the ACPI base address from EFI
    let rsdp_addr = efi::get_acpi_table().ok_or(Error::RsdpNotFound)?;

    // Read the ACPI table at the base address
    let rsdp = RsdpExtended::from_addr(rsdp_addr)?;

    // Get XSDT
    let xsdt = Table::from_addr(PhysAddr(rsdp.xsdt_address), TableType::Xsdt)?;

    println!("XSDT: {:#x?}", xsdt);

    Ok(())
}


/// Computes a checksum by caluclating the sum of all bytes in `addr..(addr + size)`,
/// returns `Ok` if `sum == 0`
unsafe fn compute_checksum(addr: PhysAddr, size: usize, r#type: TableType) -> Result<()> {

    let chk = (0..size).fold(0u8, |acc, offset| {
        acc.wrapping_add(unsafe { mm::read_physaddr::<u8>(PhysAddr(addr.0 + offset))})
    });
    if chk != 0 {
        Err(Error::ChecksumMismatch(r#type))
    } else {
        Ok(())
    }
}

fn parse_madt(paddr: PhysAddr) {}

fn parse_srat(paddr: PhysAddr) {}
