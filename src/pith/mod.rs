//! the crate holding core code central to all tasks of the operating system.
pub mod interrupts;
pub mod gdt;



/// Intialize of key kernel structures, including the IDT and more.
pub fn init() {
    interrupts::init_idt();
    gdt::init();
}
