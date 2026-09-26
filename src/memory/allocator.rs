use super::frame::PhysFrame;
use super::{Error, PhysAddr, Result};
use crate::memory::Range;

use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Bump allocator over one UEFI memory range.
///
/// Frames come from `[start, end)`, page-aligned. The heap is a different
/// slice of the same usable region (`memory::init` splits them). Running
/// out of frames is an error: returning nothing and letting the caller
/// retry just spins.
#[derive(Debug, Copy, Clone)]
pub struct FrameAllocator {
    /// First address that may still be handed out.
    next: u64,
    /// Exclusive end of the frame pool.
    end: u64,
    /// How many pages have been allocated so far.
    n: u64,
}

impl FrameAllocator {
    const FRAME_SIZE: u64 = 4096;

    /// `Err` when the range does not contain a whole 4 KiB frame.
    ///
    /// `start` is rounded up and `end` is rounded down. A range that
    /// runs backwards, or that shrinks to nothing once aligned, is rejected
    /// here instead of wrapping `size()` and walking off the end of RAM.
    pub fn new(memory_range: Range) -> Result<Self> {
        let start = crate::memory::align_up(memory_range.start, Self::FRAME_SIZE);
        let end = crate::memory::align_down(memory_range.end, Self::FRAME_SIZE);
        if start >= end {
            return Err(Error::Exhausted);
        }
        Ok(Self {
            next: start,
            end,
            n: 0,
        })
    }

    /// Allocate one 4 KiB frame from the front of the pool.
    ///
    /// `Exhausted` means the next frame would pass `end`.
    pub fn alloc_frame(&mut self) -> Result<PhysFrame> {
        let addr = self.next;
        let next = match addr.checked_add(Self::FRAME_SIZE) {
            Some(next) => next,
            None => return Err(Error::PastRange),
        };
        if next > self.end {
            return Err(Error::Exhausted);
        }
        self.next = next;
        self.n += 1;
        Ok(PhysFrame::containing_address(PhysAddr(addr)))
    }

    pub fn frames_allocated(&self) -> u64 {
        self.n
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
    fn bump_walks_the_given_range() {
        // 8192 bytes is two frames. The third call has spent the budget.
        let mut alloc = FrameAllocator::new(range(0x20_0000, 0x20_0000 + 8192)).unwrap();
        assert_eq!(
            alloc.alloc_frame().unwrap().start_address().as_u64(),
            0x20_0000
        );
        assert_eq!(
            alloc.alloc_frame().unwrap().start_address().as_u64(),
            0x20_1000
        );
        assert_eq!(alloc.alloc_frame().unwrap_err(), Error::Exhausted);
        assert_eq!(alloc.frames_allocated(), 2);
    }

    #[test_case]
    fn inverted_or_tiny_range_is_rejected() {
        // `end < start` used to wrap `size()` into a huge budget.
        assert!(FrameAllocator::new(range(0x8000, 0x1000)).is_err());
        assert!(FrameAllocator::new(range(0x1000, 0x1000)).is_err());
        // 0x1100 rounds up to 0x2000, 0x1800 rounds down to 0x1000.
        assert!(FrameAllocator::new(range(0x1100, 0x1800)).is_err());
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
