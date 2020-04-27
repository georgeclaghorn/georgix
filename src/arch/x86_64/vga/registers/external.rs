use lazy_static::*;
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static! {
    pub static ref MISCELLANEOUS_OUTPUT_REGISTER: Mutex<Register> =
        Mutex::new(unsafe { Register::new(0x3cc, 0x3c2) });
}

#[allow(dead_code)]
pub struct Register {
    reading_port: Port<u8>,
    writing_port: Port<u8>
}

impl Register {
    pub unsafe fn new(read: u16, write: u16) -> Register {
        Register {
            reading_port: Port::new(read),
            writing_port: Port::new(write)
        }
    }

    pub fn read(&mut self) -> u8 {
        unsafe { self.reading_port.read() }
    }

    pub fn write(&mut self, value: u8) {
        unsafe { self.writing_port.write(value) }
    }

    pub fn get(&mut self, index: u8) -> bool {
        self.read() & (1 << index) != 0
    }

    #[allow(dead_code)]
    pub fn set(&mut self, index: u8) {
        let value = self.read();
        self.write(value | (1 << index));
    }
}
