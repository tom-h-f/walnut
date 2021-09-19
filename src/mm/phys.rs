use super::RangeSet;
use core::mem::size_of;

#[derive(Clone, Copy, Debug)]
pub struct PhysicalMemoryMap {
    pub map: RangeSet,
    pub start: u64,
}

impl PhysicalMemoryMap {
    pub fn new(map: RangeSet) -> Self {
        let mut s = Self { map, start: 0 };
        s.start = s.map.largest().start;
        s
    }
}

/// Represents a physical address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
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

    pub fn is_empty(&self) -> bool {
        self.1 == 0
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

    /// Reads a `T` from the slice, without updating the pointer or slice.
    pub unsafe fn peek<T>(&self) -> Result<T, ()> {
        if self.1 < size_of::<T>() {
            Err(())
        } else {
            Ok(readpu::<T>(self.0))
        }
    }
}

/// Reads `T` at physical address `addr`
#[inline(always)]
pub unsafe fn readp<T>(addr: PhysAddr) -> T {
    core::ptr::read(addr.0 as *mut T)
}

/// Writes `val` at physical address `addr`
#[inline(always)]
pub unsafe fn writep<T>(addr: PhysAddr, val: T) {
    core::ptr::write(addr.0 as *mut T, val)
}

/// Reads `T` at unaligned physical address `addr`
#[inline(always)]
pub unsafe fn readpu<T>(addr: PhysAddr) -> T {
    core::ptr::read_unaligned(addr.0 as *mut T)
}

/// Writes `val` at unaligned physical address `addr`
#[inline(always)]
pub unsafe fn writepu<T>(addr: PhysAddr, val: T) {
    core::ptr::write_unaligned(addr.0 as *mut T, val)
}
