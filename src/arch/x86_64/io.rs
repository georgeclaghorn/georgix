use super::instructions::outb;

pub struct Port {
    number: u16
}

impl Port {
    pub fn new(number: u16) -> Port {
        Port { number }
    }

    pub unsafe fn write<T>(&self, byte: T) where T: WritableToPort {
        byte.write_to_port(self.number)
    }
}

pub trait WritableToPort {
    unsafe fn write_to_port(&self, number: u16);
}

impl WritableToPort for u8 {
    unsafe fn write_to_port(&self, number: u16) {
        outb(number, *self);
    }
}
