use super::MemoryRange;

pub mod linked_list;

use linked_list::LinkedListAllocator;

#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> =
    Locked::new(LinkedListAllocator::new());

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}



pub fn init_heap_allocator(memory_range: MemoryRange) {
    unsafe { ALLOCATOR.lock().init(memory_range.start, memory_range.end); }
}


pub fn init_frame_allocator() {

}



