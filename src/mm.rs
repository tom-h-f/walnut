//! Various memory management and CPU state mangement functions and macros.

/// Represents a physical address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysAddr(pub usize);

/// Reads `T` at physical address `paddr`
pub unsafe fn read_physaddr<T>(paddr: PhysAddr) -> T {
    core::ptr::read(paddr.0 as *mut T)
}

/// Writes `val` at physical address `paddr`
pub unsafe fn write_physaddr<T>(paddr: PhysAddr, val: T) {
    core::ptr::write(paddr.0 as *mut T, val)
}
