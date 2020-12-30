use lazy_static::*;
use spin::Mutex;
use crate::arch::x86_64::io::Port;

use super::external::MISCELLANEOUS_OUTPUT_REGISTER;

lazy_static! {
    pub static ref CURSOR_START_REGISTER: Mutex<Register<'static>> = Mutex::new(Register::new(&PORTS, 0x0A));

    static ref PORTS: Mutex<PortPair> =
        Mutex::new(
            PortPair {
                address_port: Port::new(*BASE + 4),
                data_port: Port::new(*BASE + 5)
            }
        );

    static ref BASE: u16 =
        if MISCELLANEOUS_OUTPUT_REGISTER.lock().get(0) {
            0x03D0
        } else {
            0x03B0
        };
}

pub struct Register<'a> {
    ports: &'a Mutex<PortPair>,
    index: u8
}

impl<'a> Register<'a> {
    fn new(ports: &Mutex<PortPair>, index: u8) -> Register {
        Register { ports, index }
    }

    pub fn read(&self) -> u8 {
        unsafe { self.ports.lock().read_from(self.index) }
    }

    pub fn write(&self, value: u8) {
        unsafe { self.ports.lock().write_to(self.index, value) }
    }

    #[allow(dead_code)]
    pub fn get(&self, index: u8) -> bool {
        self.read() & (1 << index) != 0
    }

    pub fn set(&self, index: u8) {
        self.write(self.read() | (1 << index))
    }
}

pub struct PortPair {
    address_port: Port,
    data_port: Port
}

impl PortPair {
    unsafe fn read_from(&self, index: u8) -> u8 {
        self.address_port.write(index);
        self.data_port.read()
    }

    unsafe fn write_to(&self, index: u8, value: u8) {
        self.address_port.write(index);
        self.data_port.write(value);
    }
}
