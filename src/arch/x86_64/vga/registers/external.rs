use lazy_static::*;
use spin::Mutex;
use crate::arch::x86_64::io::Port;

lazy_static! {
    pub static ref MISCELLANEOUS_OUTPUT_REGISTER: Mutex<Register> =
        Mutex::new(
            Register {
                reading_port: Port::new(0x3CC),
                writing_port: Port::new(0x3C2)
            }
        );
}

#[allow(dead_code)]
pub struct Register {
    reading_port: Port,
    writing_port: Port
}

impl Register {
    pub fn read(&self) -> u8 {
        unsafe { self.reading_port.read() }
    }

    pub fn write(&self, value: u8) {
        unsafe { self.writing_port.write(value) }
    }

    pub fn get(&self, index: u8) -> bool {
        self.read() & (1 << index) != 0
    }

    #[allow(dead_code)]
    pub fn set(&self, index: u8) {
        self.write(self.read() | (1 << index));
    }
}
