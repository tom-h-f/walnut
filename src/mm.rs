//! Various memory management and CPU state mangement functions and macros.


/// Represents a physical address
pub struct PhysAddr(pub usize);

/// Reads `T` at physical address `paddr`
pub unsafe fn read_physaddr<T>(paddr: PhysAddr) -> T {
    core::ptr::read_volatile(paddr.0 as *mut T)
}

/// Writes `val` at physical address `paddr`
pub unsafe fn write_physaddr<T>(paddr: PhysAddr, val: T) {
    core::ptr::write_volatile(paddr.0 as *mut T, val)
}

