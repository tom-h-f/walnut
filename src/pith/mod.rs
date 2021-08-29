//! the crate holding core code central to all tasks of the operating system.
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;



/// Intialize of key kernel structures, including the IDT and more.
pub fn init() {
    interrupts::init_idt();
    interrupts::init_pics();
    gdt::init();
    // executes `sti`
    x86_64::instructions::interrupts::enable();
}
