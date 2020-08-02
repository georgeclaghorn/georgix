use crate::{println, print};
use super::idt::{InterruptStackFrame, PageFaultErrorCode};
use super::complete;

pub extern "x86-interrupt" fn breakpoint(stack_frame: &InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault(stack_frame: &InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn general_protection_fault(stack_frame: &InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT ({})\n{:?}", error_code, stack_frame);
}

pub extern "x86-interrupt" fn page_fault(stack_frame: &InterruptStackFrame, error_code: PageFaultErrorCode) {
    panic!("EXCEPTION: PAGE FAULT ({:?})\n{:?}", error_code, stack_frame);
}

pub extern "x86-interrupt" fn timer(_stack_frame: &InterruptStackFrame) {
    print!(".");
    complete();
}
