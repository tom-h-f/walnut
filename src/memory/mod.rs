
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
