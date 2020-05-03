use x86_64::structures::idt::InterruptStackFrame;
use crate::println;

pub extern "x86-interrupt" fn breakpoint(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
