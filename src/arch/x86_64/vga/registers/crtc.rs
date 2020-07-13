use lazy_static::*;
use spin::Mutex;
use crate::arch::x86_64::io::Port;

use super::external::MISCELLANEOUS_OUTPUT_REGISTER;

lazy_static! {
    pub static ref CURSOR_START_REGISTER: Mutex<Register> =
        Mutex::new(unsafe { Register::new(&ADDRESS_PORT, &DATA_PORT, 0x0A) });

    static ref ADDRESS_PORT: Mutex<Port> = Mutex::new(Port::new(*BASE + 4));
    static ref DATA_PORT: Mutex<Port> = Mutex::new(Port::new(*BASE + 5));

    static ref BASE: u16 =
        if MISCELLANEOUS_OUTPUT_REGISTER.lock().get(0) {
            0x03D0
        } else {
            0x03B0
        };
}

pub struct Register {
    address_port: &'static Mutex<Port>,
    data_port: &'static Mutex<Port>,
    index: u8
}

impl Register {
    pub unsafe fn new(address_port: &'static Mutex<Port>, data_port: &'static Mutex<Port>, index: u8) -> Register {
        Register {
            address_port,
            data_port,
            index
        }
    }

    pub fn read(&self) -> u8 {
        unsafe {
            self.address_port.lock().write(self.index);
            self.data_port.lock().read()
        }
    }

    pub fn write(&mut self, value: u8) {
        unsafe {
            self.address_port.lock().write(self.index);
            self.data_port.lock().write(value)
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, index: u8) -> bool {
        self.read() & (1 << index) != 0
    }

    pub fn set(&mut self, index: u8) {
        self.write(self.read() | (1 << index));
    }
}
