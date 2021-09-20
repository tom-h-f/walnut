pub mod paging;

#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryRange {
    pub start: u64,
    pub end:   u64,
}

impl MemoryRange {
    pub fn new() -> Self {
        MemoryRange { start: 0, end: 0 }
    }
    pub fn size(&self) -> u64 {
        self.end - self.start
    }
}

/// Represents a physical address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct PhysAddr(pub u64);

/// Represents a physical address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct VirtAddr(pub u64);

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

/// Initializes memory structures and allocators, linearly in the form:
/// ```text
/// Re-Identity Map Memory via UEFI
///             🠗
/// Intialize Page Frame Allocator
///             🠗
/// Intialize Heap Allocator
/// ```
pub fn init() {

}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocator Error {:#x?}", layout);
}
