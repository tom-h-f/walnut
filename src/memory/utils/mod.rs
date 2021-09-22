pub mod addr;
pub mod rangeset;

use super::Addr;



pub use rangeset::{Range, RangeSet};

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    AddressNotAligned,
}

/// Align address upwards.
///
/// Returns the smallest x with alignment `align` so that x >= addr. The alignment must be
/// a power of 2.
#[inline]
pub fn align_up<U>(addr: u64, align: U) -> u64
where
    U: Into<u64>,
{
    let align: u64 = align.into();
    assert!(align.is_power_of_two(), "`align` must be a power of two");
    let align_mask = align - 1;
    if addr & align_mask == 0 {
        addr // already aligned
    } else {
        (addr | align_mask) + 1
    }
}
/// Align address downwards.
///
/// Returns the greatest x with alignment `align` so that x <= addr. The alignment must be
/// a power of 2.
#[inline]
pub fn align_down<U>(addr: u64, align: U) -> u64
where
    U: Into<u64>,
{
    let align: u64 = align.into();
    assert!(align.is_power_of_two(), "`align` must be a power of two");
    addr & !(align - 1)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocator Error {:#x?}", layout);
}

/// Reads `T` at physical address `addr`
#[inline(always)]
pub unsafe fn readp<A, T>(addr: A) -> T where A: Addr {
    core::ptr::read(addr.as_u64() as *mut T)
}

/// Writes `val` at physical address `addr`
#[inline(always)]
pub unsafe fn writep<A, T>(addr: A, val: T) where A: Addr {
    core::ptr::write(addr.as_u64() as *mut T, val)
}

/// Reads `T` at unaligned physical address `addr`
#[inline(always)]
pub unsafe fn readpu<A, T>(addr: A) -> T  where A: Addr {
    core::ptr::read_unaligned(addr.as_u64() as *mut T)
}

/// Writes `val` at unaligned physical address `addr`
#[inline(always)]
pub unsafe fn writepu<A, T>(addr: A, val: T) where A: Addr {
    core::ptr::write_unaligned(addr.as_u64() as *mut T, val)
}