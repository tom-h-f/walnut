pub mod addr;
pub mod rangeset;

use super::Addr;

pub use rangeset::{Range, RangeSet};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum Error {
    AddressNotAligned,
    CouldntAllocFrame,
    /// Every frame that fits in the allocator's range has been handed out.
    Exhausted,
    /// The next frame address is past the end of the backing range.
    PastRange,
    Unknown(u64),
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
pub unsafe fn readp<A, T>(addr: A) -> T
where
    A: Addr,
{
    core::ptr::read(addr.as_u64() as *mut T)
}

/// Writes `val` at physical address `addr`
#[inline(always)]
pub unsafe fn writep<A, T>(addr: A, val: T)
where
    A: Addr,
{
    core::ptr::write(addr.as_u64() as *mut T, val)
}

/// Reads `T` at unaligned physical address `addr`
#[inline(always)]
pub unsafe fn readpu<A, T>(addr: A) -> T
where
    A: Addr,
{
    core::ptr::read_unaligned(addr.as_u64() as *mut T)
}

/// Writes `val` at unaligned physical address `addr`
#[inline(always)]
pub unsafe fn writepu<A, T>(addr: A, val: T)
where
    A: Addr,
{
    core::ptr::write_unaligned(addr.as_u64() as *mut T, val)
}

#[cfg(test)]
mod tests {
    use super::{align_down, align_up};
    use crate::PhysAddr;

    #[test_case]
    fn align_down_stays_on_this_page() {
        assert_eq!(align_down(0x1000, 0x1000u64), 0x1000);
        assert_eq!(align_down(0x1fff, 0x1000u64), 0x1000);
    }

    #[test_case]
    fn align_up_reaches_the_next_page() {
        // An aligned address must not move. One byte past a page has to
        // skip to the next one, or a PTE would store flag bits as part
        // of the address.
        assert_eq!(align_up(0x1000, 0x1000u64), 0x1000);
        assert_eq!(align_up(0x1001, 0x1000u64), 0x2000);
    }

    #[test_case]
    fn truncate_drops_bits_above_the_52_bit_mask() {
        // x86_64 physical addresses are 52 bits. Bits 52..64 are not part
        // of the address (they used to be reserved, and in a PTE they
        // overlap flag bits), so they are discarded.
        let high = (1u64 << 52) + 0xabc;
        assert_eq!(PhysAddr::new_truncate(high).as_u64(), 0xabc);
        assert_eq!(PhysAddr::new_truncate(0xabc).as_u64(), 0xabc);
    }
}
