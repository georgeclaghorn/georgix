use lazy_static::*;
use x86_64::instructions::port::Port;
use spin::Mutex;

lazy_static! {
    pub static ref CURSOR_START_REGISTER: Mutex<Register> = Mutex::new(unsafe { Register::new(0x0a) });

    static ref ADDRESS_PORT: Mutex<Port<u8>> = Mutex::new(Port::new(*BASE + 4));
    static ref DATA_PORT: Mutex<Port<u8>> = Mutex::new(Port::new(*BASE + 5));

    static ref BASE: u16 = {
        if super::MISCELLANEOUS_OUTPUT_REGISTER.lock().get(0) {
            0x03d0
        } else {
            0x03b0
        }
    };
}

pub struct Register {
    index: u8
}

impl Register {
    pub unsafe fn new(index: u8) -> Register {
        Register { index }
    }

    pub fn read(&self) -> u8 {
        unsafe {
            ADDRESS_PORT.lock().write(self.index);
            DATA_PORT.lock().read()
        }
    }

    pub fn write(&self, value: u8) {
        unsafe {
            ADDRESS_PORT.lock().write(self.index);
            DATA_PORT.lock().write(value)
        }
    }

    pub fn get(&self, index: u8) -> bool {
        self.read() & (1 << index) != 0
    }

    pub fn set(&self, index: u8) {
        self.write(self.read() | (1 << index));
    }
}
