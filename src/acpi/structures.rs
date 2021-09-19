use crate::mm::{self, PhysAddr};
use core::mem::size_of;
use core::fmt;


pub mod apic;

pub mod madt;
pub use madt::Madt;

pub mod rsdp;
pub use rsdp::{Rsdp, RsdpExtended};

// A `Result` type that wraps an ACPI error
pub type Result<T> = core::result::Result<T, Error>;

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

    LengthMismatch(TableType),

    /// The XSDT table size was not evenly divisble by the array element size
    XsdtBadEntries,

    /// An integer overflow occurred
    IntegerOverflow,

    /// An integer underflow occurred
    IntegerUnderflow,
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

    /// Multiple APIC Description Table
    Madt,

    /// System/Static Resource Affinity Table
    Srat,

    // An unknown Table Type
    Unknown([u8; 4]),
}

impl From<[u8; 4]> for TableType {
    fn from(v: [u8; 4]) -> Self {
        match &v {
            b"XSDT" => Self::Xsdt,
            b"APIC" => Self::Madt,
            b"SRAT" => Self::Srat,
            b"RSDP" => Self::Rsdp,
            _ => Self::Unknown(v),
        }
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
        f.debug_struct("ACPI Table")
            .field("Signature", &core::str::from_utf8(&self.signature).unwrap())
            .field("Length", &self.length)
            .field("Revision", &self.revision)
            .field("OEM ID", &core::str::from_utf8(&self.oem_id).unwrap())
            .field(
                "OEM Table ID",
                &core::str::from_utf8(&self.oem_table_id).unwrap(),
            )
            // I don't know if creator ID is actually a string but its more fun this way
            .field(
                "Creator ID",
                &core::str::from_utf8(&self.creator_id.to_le_bytes()).unwrap(),
            )
            .field("Creator Revision", &self.creator_revision)
            .finish()
    }
}

impl Table {
    /// Attempts to process `addr` as an ACPI table.
    ///
    /// # Returns
    /// `Ok(table, table_type, addr, size)` - `table` is the processed `Table` struct,
    /// `table_type` is the `TableType` of `table`,
    /// `addr` is the address of the payload of `table` and
    /// `size` is the size of `table`'s payload.
    /// `Err(err)` - An `acpi::structures::Error` indicating what went wrong
    pub unsafe fn from_addr(addr: PhysAddr) -> Result<(Self, TableType, PhysAddr, usize)> {
        // Read the table
        let table = mm::readp::<Self>(addr);

        // Get the type of this table
        let r#type = TableType::from(table.signature);

        compute_checksum(addr, table.length as usize, r#type)?;

        let header_size = size_of::<Self>();

        // Compute the address of the table's payload and
        // the size of it in bytes
        let payload_size = (table.length as usize)
            .checked_sub(header_size)
            .ok_or(Error::LengthMismatch(r#type))?;
        let payload_addr = PhysAddr(
            addr.0
                .checked_add(header_size)
                .ok_or(Error::IntegerOverflow)?,
        );

        Ok((table, r#type, payload_addr, payload_size))
    }
}
#[repr(C, packed)]
pub struct Rsdt {
    table: Table,
    /// this points to Other SDT
    other_sdt_ptr: usize,
}
/// Computes a checksum by caluclating the sum of all bytes in `addr..(addr + size)`,
/// returns `Ok` if `sum == 0`
unsafe fn compute_checksum(addr: PhysAddr, size: usize, r#type: TableType) -> Result<()> {
    let chk = (0..size).fold(0u8, |acc, offset| {
        acc.wrapping_add(unsafe { mm::readp::<u8>(PhysAddr(addr.0 + offset)) })
    });
    if chk != 0 {
        Err(Error::ChecksumMismatch(r#type))
    } else {
        Ok(())
    }
}
