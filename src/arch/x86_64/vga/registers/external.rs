use lazy_static::*;
use spin::Mutex;
use crate::arch::x86_64::io::Port;

lazy_static! {
    pub static ref MISCELLANEOUS_OUTPUT_REGISTER: Mutex<Register> =
        Mutex::new(unsafe { Register::new(0x3CC, 0x3C2) });
}

#[allow(dead_code)]
pub struct Register {
    reading_port: Port,
    writing_port: Port
}

impl Register {
    pub unsafe fn new(read: u16, write: u16) -> Register {
        Register {
            reading_port: Port::new(read),
            writing_port: Port::new(write)
        }
    }

    pub fn read(&self) -> u8 {
        unsafe { self.reading_port.read() }
    }

    pub fn write(&mut self, value: u8) {
        unsafe { self.writing_port.write(value) }
    }

    pub fn get(&self, index: u8) -> bool {
        self.read() & (1 << index) != 0
    }

    #[allow(dead_code)]
    pub fn set(&mut self, index: u8) {
        self.write(self.read() | (1 << index));
    }
}
