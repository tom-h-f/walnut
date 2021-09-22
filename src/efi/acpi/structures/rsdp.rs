use super::{compute_checksum, Error, Result, TableType};
use crate::mm::{self, PhysAddr};
use core::mem::size_of;

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
    pub unsafe fn from_addr(addr: PhysAddr) -> Result<Self> {
        let rsdp = mm::readp::<Rsdp>(addr);

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
    pub descriptor: Rsdp,
    pub length: u32,
    pub xsdt_address: usize,
    pub extended_checksum: u8,
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
        let rsdp = mm::readp::<RsdpExtended>(addr);

        rsdp.check_length()?;

        compute_checksum(addr, size_of::<Self>(), TableType::RsdpExtended)?;

        Ok(rsdp)
    }

    fn check_length(&self) -> Result<()> {
        if self.length as usize != size_of::<RsdpExtended>() {
            return Err(Error::RsdpExtendedSizeMisMatch);
        }
        Ok(())
    }
}

impl fmt::Debug for RsdpExtended {
    #[allow(unaligned_references)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RSDP (for ACPI >= 2.0)")
            .field("", &self.descriptor)
            .field("length", &self.length)
            .field("xsdt_address", unsafe {
                &core::ptr::read_unaligned(&self.xsdt_address)
            })
            .field("extended_checksum", &self.extended_checksum)
            .finish()
    }
}
