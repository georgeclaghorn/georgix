#![cfg(test)]

pub mod console {
    use lazy_static::lazy_static;
    use spin::Mutex;
    use uart_16550::SerialPort;
    use core::fmt::Write;

    lazy_static! {
        static ref COM1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });
    }

    pub fn initialize() {
        COM1.lock().init();
    }

    pub fn print(args: core::fmt::Arguments) {
        COM1.lock().write_fmt(args).unwrap();
    }
}

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
