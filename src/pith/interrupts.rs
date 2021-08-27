use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", frame);
}

extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, _errcode: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\nErrorCode: {}\n{:#?}", _errcode, frame);
}

// this tests exception handling of breakpoints
// by invoking a breakpoint exception and execution continuing
// if it doesn't QEMU will return an exit code and cause that to fail the test.
#[test_case]
fn test_exception_breakpoint() {
    unsafe {
        asm!("int3");
    }
}

