use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use crate::{println, print};
use super::{Index, end};

pub extern "x86-interrupt" fn breakpoint(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn general_protection_fault(stack_frame: &mut InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT ({})\n{:?}", error_code, stack_frame);
}

pub extern "x86-interrupt" fn page_fault(stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode) {
    panic!("EXCEPTION: PAGE FAULT ({:?})\n{:?}", error_code, stack_frame);
}

pub extern "x86-interrupt" fn timer(_stack_frame: &mut InterruptStackFrame) {
    print!(".");
    end(Index::Timer);
}
