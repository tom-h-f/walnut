#![no_std]
#![no_main]
#![feature(asm, abi_efiapi, llvm_asm, bool_to_option)]
#![feature(panic_info_message)]
#![feature(abi_x86_interrupt)]
#![allow(clippy::missing_safety_doc)]
pub mod acpi;
pub mod arch;
#[doc(hidden)]
mod core_fns;
pub mod cpu;
#[doc(hidden)]
mod efi;
mod error;
pub mod mm;
pub mod paging;
pub use crate::arch::idt;
pub mod serial;


use x86_64::{VirtAddr, structures::paging::{Translate, Page}};
pub fn kmain(mut memory_map: mm::PhysicalMemoryMap) {
    arch::idt::init();
    let mut mapper = paging::init();
    let mut fa = paging::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::zero());
    paging::create_example_mapping(page, &mut mapper, &mut fa);
    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
       memory_map.map.largest().start, 
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { mapper.translate_addr(virt) };
        println!("{:?} -> {:?}", virt, phys);
    }
    panic!("reached end of kmain()");
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    println!("!!! PANIC !!!");
    if let Some(loc) = _info.location() {
        println!("Location: {}", loc);
    }
    if let Some(msg) = _info.message() {
        println!("Message:  {}", msg);
    }
    dump_state!();
    loop {}
}
