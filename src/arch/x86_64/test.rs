#![cfg(test)]

use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum ExitCode {
    Success = 0x10,
    Failure = 0x11
}

pub fn exit(code: ExitCode) -> ! {
    unsafe {
        Port::new(0xF4).write(code as u32);
        core::hint::unreachable_unchecked();
    }
}
