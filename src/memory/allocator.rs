use super::frame::PhysFrame;
use super::PhysAddr;
use crate::memory::Range;

use linked_list_allocator::LockedHeap;


#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();



/// Physical frame allocator that works from the UEFI provided memory map
#[derive(Debug, Copy, Clone)]
pub struct FrameAllocator {
    memory_range: Range,
    next_frame: PhysFrame,
    /// How many pages have been allocated so far.
    n: u64,
}

impl FrameAllocator {
    const FRAME_SIZE: u64 = 4096;
    pub fn new(memory_range: Range) -> Self {

        let start_frame = PhysFrame::containing_address(PhysAddr(0x1000));
        Self {
            memory_range,
            next_frame: start_frame,
            n: 0,
        }
    }

    // TODO these functions should return a result with why,
    // since if we run out of memory we will just keep fucking
    // calling
    pub fn alloc_frame(&mut self) -> Option<PhysFrame> {
        let next = self.get_next()?;
        self.n += 1;
        Some(next)
    }

    /// Returns the next frame
    /// does not advance `self.n`
    fn get_next(&mut self) -> Option<PhysFrame> {

        // Store the 'next_frame' before we create a new one
        let frame_to_return = self.next_frame;


        // Make sure we dont allocate further than we can.
        if self.n >= (self.memory_range.size() / Self::FRAME_SIZE) {
            return None
        } 

        let frame_addr = self.memory_range.start + ((self.n + 1) * Self::FRAME_SIZE);

        if frame_addr > self.memory_range.end {
            return None;
        }

        self.next_frame = PhysFrame::containing_address(PhysAddr(frame_addr));

        // Only actually advance this if we going to succeed 

        Some(frame_to_return)
    }

}


#[cfg(test)]
mod test {
    use crate::Box;
    use crate::vec::Vec;
    #[test_case]
    fn simple_allocation() {
        let heap_value_1 = Box::new(41);
        let heap_value_2 = Box::new(13);
        assert_eq!(*heap_value_1, 41);
        assert_eq!(*heap_value_2, 13);
    }

    #[test_case]
    fn large_vec() {
        let n = 1000;
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(i);
        }
        assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    }

    #[test_case]
    fn many_boxes() {
        for i in 0..0x4444 {
            let x = Box::new(i);
            assert_eq!(*x, i);
        }
    }
}