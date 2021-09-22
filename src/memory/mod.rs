pub mod frame;
pub mod paging;
pub mod utils;

pub use utils::{
    addr::{Addr, PhysAddr, VirtAddr, PhysSlice},
    align_down, align_up,
    rangeset::{Range, RangeSet},
    Error, Result, readp, readpu, writep, writepu
};

pub fn init_heap_allocator(memory_range: Range) {
    unsafe {
        paging::ALLOCATOR
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
