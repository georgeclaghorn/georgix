use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use core::fmt::Write;

lazy_static! {
    pub static ref COM1: Mutex<SerialPort> = {
        let mut port = unsafe { SerialPort::new(0x3F8) };
        port.init();
        Mutex::new(port)
    };
}

pub fn print(args: core::fmt::Arguments) {
    COM1.lock().write_fmt(args).unwrap();
}
