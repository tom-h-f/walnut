//! Various memory management and CPU state mangement functions and macros.

#![allow(clippy::result_unit_err)]
// TODO we should probably implement a custom
// `Result` and `Error` type for this module

pub mod phys;
pub mod rangeset;
pub mod virt;

pub use phys::{readp, readpu, writep, writepu, PhysAddr, PhysSlice, PhysicalMemoryMap};
pub use rangeset::{Range, RangeSet};
pub use virt::{readv, readvu, writev, writevu, VirtAddr};

/// Align address downwards.
///
/// Returns the greatest x with alignment `align` so that x <= addr. The alignment must be
///  a power of 2.
#[inline]
pub fn align_down(addr: u64, align: u64) -> u64 {
    assert!(align.is_power_of_two(), "`align` must be a power of two");
    addr & !(align - 1)
}

/// Align address upwards.
///
/// Returns the smallest x with alignment `align` so that x >= addr. The alignment must be
/// a power of 2.
#[inline]
pub fn align_up(addr: u64, align: u64) -> u64 {
    assert!(align.is_power_of_two(), "`align` must be a power of two");
    let align_mask = align - 1;
    if addr & align_mask == 0 {
        addr // already aligned
    } else {
        (addr | align_mask) + 1
    }
}
// TODO implement allocator here
