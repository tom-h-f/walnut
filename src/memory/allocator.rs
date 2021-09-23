use super::paging::{Locked, linked_list::LinkedListAllocator};
use super::frame::PhysFrame;
use super::PhysAddr;
use crate::memory::RangeSet;

#[global_allocator]
pub static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());



/// Physical frame allocator that works from the UEFI provided memory map
#[derive(Debug, Copy, Clone)]
pub struct FrameAllocator {
    memory_map: RangeSet,
    next_frame: PhysFrame,
}

impl FrameAllocator {

    pub fn new(memory_map: RangeSet) -> Self {

        let start_frame = PhysFrame::containing_address(PhysAddr(0x1000));
        Self {
            memory_map,
            next_frame: start_frame,
        }
    }

}
