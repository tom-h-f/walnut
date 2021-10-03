pub mod frame;
pub mod paging;
pub mod utils;
pub mod allocator;

pub use utils::{
    addr::{Addr, PhysAddr, VirtAddr, PhysSlice},
    align_down, align_up,
    rangeset::{Range, RangeSet},
    Error, Result, 
    readp, readpu, writep, writepu
};

use self::allocator::{ALLOCATOR, FrameAllocator};

pub fn init_heap_allocator(memory_range: Range, _frame_allocator: &mut FrameAllocator) {
    unsafe {
        ALLOCATOR.lock().init(memory_range.start as usize, memory_range.end as usize)
    }
}

pub fn init_frame_allocator() {


}

/// Initializes memory structures and allocators, linearly in the form:
/// ```text
/// Re-Identity Map Memory via UEFI
///             🠗
/// Intialize Page Frame Allocator
///             🠗
/// Intialize Heap Allocator
/// ```
pub fn init(biggest_region: Range) -> Option<FrameAllocator> {

    let mut frame_allocator = FrameAllocator::new(biggest_region);
    init_heap_allocator(biggest_region, &mut frame_allocator);

    Some(frame_allocator)
}
