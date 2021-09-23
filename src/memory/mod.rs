pub mod frame;
pub mod paging;
pub mod utils;
pub mod allocator;

pub use utils::{
    addr::{Addr, PhysAddr, VirtAddr},
    align_down, align_up,
    rangeset::{Range, RangeSet},
    Error, Result,
};

pub fn init_heap_allocator(memory_range: Range) {
    unsafe {
        allocator::ALLOCATOR
            .lock()
            .init(memory_range.start, memory_range.end);
    }
}

pub fn init_frame_allocator() {}

/// Initializes memory structures and allocators, linearly in the form:
/// ```text
/// Re-Identity Map Memory via UEFI
///             🠗
/// Intialize Page Frame Allocator
///             🠗
/// Intialize Heap Allocator
/// ```
pub fn init() {}
