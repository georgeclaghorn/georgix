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
        COM1.lock().init()
    }

    pub fn print(args: core::fmt::Arguments) {
        crate::arch::x86_64::interrupts::suppress(|| COM1.lock().write_fmt(args).unwrap())
    }
}

use super::io::Port;

pub fn exit(status: u32) -> ! {
    unsafe {
        Port::new(0xF4).write(status + 0x10);
        core::hint::unreachable_unchecked();
    }
}
