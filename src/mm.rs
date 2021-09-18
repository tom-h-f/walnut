//! Various memory management and CPU state mangement functions and macros.

use core::mem::size_of;

/// Represents a physical address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysSlice(pub PhysAddr, pub usize);

impl PhysSlice {
    /// Create a new slice to physical memory
    /// Returns `None` if we encounter an overflow or other
    /// error in the address range asked for.
    pub unsafe fn new(addr: PhysAddr, size: usize) -> Self {
        Self(addr, size)
    }

    /// Get the remaining length of the slice
    pub fn len(&self) -> usize {
        self.1
    }

    /// Discard `bytes` from the slice by updating the pointer and length
    pub fn discard(&mut self, bytes: usize) -> Result<(), ()> {
        if self.1 >= bytes {
            // Update length
            self.1 -= bytes;
            // and pointer
            (self.0).0 += bytes;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Read a `T` from the slice, updating the pointer and size.
    pub unsafe fn consume<T>(&mut self) -> Result<T, ()> {
        if self.1 < size_of::<T>() {
            return Err(());
        }

        // read the data
        let data = readpu::<T>(self.0);

        // Update length
        self.1 -= size_of::<T>();
        // and pointer
        (self.0).0 += size_of::<T>();

        Ok(data)
    }
}

/// Reads `T` at physical address `paddr`
#[inline(always)]
pub unsafe fn readp<T>(paddr: PhysAddr) -> T {
    core::ptr::read(paddr.0 as *mut T)
}

/// Writes `val` at physical address `paddr`
#[inline(always)]
pub unsafe fn writep<T>(paddr: PhysAddr, val: T) {
    core::ptr::write(paddr.0 as *mut T, val)
}

/// Reads `T` at unaligned physical address `paddr`
#[inline(always)]
pub unsafe fn readpu<T>(paddr: PhysAddr) -> T {
    core::ptr::read_unaligned(paddr.0 as *mut T)
}

/// Writes `val` at unaligned physical address `paddr`
#[inline(always)]
pub unsafe fn writepu<T>(paddr: PhysAddr, val: T) {
    core::ptr::write_unaligned(paddr.0 as *mut T, val)
}


// TODO implement allocator here