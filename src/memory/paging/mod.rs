//! Read the page tables UEFI already installed.
//!
//! Long mode is on, and CR3 holds the physical address of the level-4
//! table (PML4). Each level is 512 entries of 8 bytes. The low 12 bits of
//! an entry are flags, not part of the address:
//!
//! * bit 0 present
//! * bit 1 writable
//! * bit 2 user
//! * bit 3 write-through
//! * bit 4 cache disable
//! * bit 5 accessed
//! * bit 6 dirty
//! * bit 7 page size (huge page at levels 3 and 2)
//! * bit 8 global
//! * bit 63 no-execute
//!
//! Those bits live in the `x86_64` crate's `PageTableEntry`. This module
//! does not build tables. `active_level_4_table` adds `physical_memory_offset`
//! because, after `SetVirtualAddressMap`, the PML4 frame is reached at its
//! relocated virtual address. Using the raw physical address would miss it.

pub mod linked_list;
pub mod page;

pub use super::{Error, Result};

pub const PAGE_SIZE: u64 = 4096;

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

use x86_64::{structures::paging::PageTable, VirtAddr};

/// Unused. The kernel keeps the page tables UEFI built.
pub fn init() {}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
