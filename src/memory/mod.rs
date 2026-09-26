pub mod allocator;
pub mod frame;
pub mod paging;
pub mod utils;

pub use utils::{
    addr::{Addr, PhysAddr, PhysSlice, VirtAddr},
    align_down, align_up,
    rangeset::{Range, RangeSet},
    readp, readpu, writep, writepu, Error, Result,
};

use self::allocator::{FrameAllocator, ALLOCATOR};

/// Heap carved from the bottom of the largest usable range.
///
/// One mebibyte is enough for the boot allocations (`Box`, a small `Vec`).
/// The rest of the range is the frame pool. A region smaller than this plus
/// one frame gives half of itself to the heap, still page-aligned.
const HEAP_BYTES: u64 = 1024 * 1024;

pub fn init_heap_allocator(memory_range: Range) {
    // `LockedHeap::init` takes a bottom address and a size. Passing `end`
    // as the size makes the heap run `start` bytes past the range.
    let size = (memory_range.end - memory_range.start) as usize;
    unsafe {
        ALLOCATOR.lock().init(memory_range.start as usize, size);
    }
}

/// Split `biggest_region` into a heap and a frame pool.
///
/// The range is the largest usable UEFI descriptor (boot services or
/// conventional memory), with an exclusive end. Both pieces are 4 KiB
/// aligned. The heap is placed first so a frame allocation cannot hand
/// out a page the global allocator is already using.
pub fn init(biggest_region: Range) -> Result<FrameAllocator> {
    let start = align_up(biggest_region.start, 4096u64);
    let end = align_down(biggest_region.end, 4096u64);
    if end <= start {
        return Err(Error::Exhausted);
    }
    let avail = end - start;
    let heap_bytes = if avail > HEAP_BYTES + 4096 {
        HEAP_BYTES
    } else {
        align_down(avail / 2, 4096u64)
    };
    if heap_bytes < 4096u64 || heap_bytes >= avail {
        return Err(Error::Exhausted);
    }
    let heap = Range {
        start,
        end: start + heap_bytes,
        descriptor: biggest_region.descriptor,
    };
    let frames = Range {
        start: start + heap_bytes,
        end,
        descriptor: biggest_region.descriptor,
    };
    init_heap_allocator(heap);
    FrameAllocator::new(frames)
}
