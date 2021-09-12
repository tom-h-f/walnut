#[derive(Debug)]
pub struct Info {
    pub best_no_of_pages: usize,
    pub best_alloc_start: usize,
}

/// The main code that runs the entire operating system.
/// it is called after UEFI intializes and calls it.
pub fn main(_info: Info) -> ! {
    init();

    loop {}
}

/// intializes various essential functions of the
/// operating system
fn init() {
    unsafe {
        asm!("cli"); // disable interrupts for now
    }
}
