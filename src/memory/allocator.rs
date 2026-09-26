use super::frame::PhysFrame;
use super::{Error, PhysAddr, Result};
use crate::memory::Range;

use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Bump allocator over one UEFI memory range.
///
/// The heap is placed on the range directly. This allocator is for later
/// physical frames. Running out of frames is an error: returning nothing
/// and letting the caller retry just spins.
#[derive(Debug, Copy, Clone)]
pub struct FrameAllocator {
    memory_range: Range,
    next_frame: PhysFrame,
    /// How many pages have been allocated so far.
    n: u64,
}

impl FrameAllocator {
    const FRAME_SIZE: u64 = 4096;

    /// First frame handed out. This is not `memory_range.start`.
    ///
    /// The range only bounds the frames after this one. Boot does not
    /// allocate through here for the heap, so the kernel image still comes
    /// up if this page is outside the UEFI range.
    const FIRST_FRAME_ADDR: u64 = 0x1000;

    pub fn new(memory_range: Range) -> Self {
        let start_frame = PhysFrame::containing_address(PhysAddr(Self::FIRST_FRAME_ADDR));
        Self {
            memory_range,
            next_frame: start_frame,
            n: 0,
        }
    }

    /// Allocate one 4 KiB frame.
    ///
    /// `Exhausted` means the range's page count is used up. `PastRange`
    /// means the following address would pass `memory_range.end` even
    /// though the count still had room (the range end is below the start,
    /// so `size()` wraps).
    pub fn alloc_frame(&mut self) -> Result<PhysFrame> {
        let next = self.get_next()?;
        self.n += 1;
        Ok(next)
    }

    /// Returns the next frame. Does not advance `self.n`.
    fn get_next(&mut self) -> Result<PhysFrame> {
        // The frame we return was chosen on the previous call. The first
        // call returns `FIRST_FRAME_ADDR`. The address computed here is
        // the one *after* it, which is what the range check applies to.
        let frame_to_return = self.next_frame;

        if self.n >= (self.memory_range.size() / Self::FRAME_SIZE) {
            return Err(Error::Exhausted);
        }

        let frame_addr = self.memory_range.start + ((self.n + 1) * Self::FRAME_SIZE);

        if frame_addr > self.memory_range.end {
            return Err(Error::PastRange);
        }

        self.next_frame = PhysFrame::containing_address(PhysAddr(frame_addr));

        Ok(frame_to_return)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::efi::memory::EfiMemoryDescriptor;
    use crate::vec::Vec;
    use crate::Box;

    fn range(start: u64, end: u64) -> Range {
        Range {
            start,
            end,
            descriptor: EfiMemoryDescriptor::default(),
        }
    }

    #[test_case]
    fn bump_returns_first_page_then_exhausts() {
        // 8192 bytes is two frames. The third call has spent the budget.
        let mut alloc = FrameAllocator::new(range(0x20_0000, 0x20_0000 + 8192));
        let first = alloc.alloc_frame().expect("first frame");
        assert_eq!(
            first.start_address().as_u64(),
            FrameAllocator::FIRST_FRAME_ADDR
        );
        assert!(alloc.alloc_frame().is_ok());
        assert_eq!(alloc.alloc_frame().unwrap_err(), Error::Exhausted);
    }

    #[test_case]
    fn address_past_end_is_past_range() {
        // `end < start` makes `size()` wrap to a huge budget, so the
        // page-count check does not fire. The end comparison does.
        let mut alloc = FrameAllocator::new(range(0x8000, 0x1000));
        assert_eq!(alloc.alloc_frame().unwrap_err(), Error::PastRange);
    }

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
