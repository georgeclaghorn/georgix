use super::instructions::outb;

pub struct Port {
    number: u16
}

impl Port {
    pub fn new(number: u16) -> Port {
        Port { number }
    }

    pub unsafe fn write<T>(&self, data: T) where T: Output {
        data.write_to(&self)
    }
}

pub trait Output {
    unsafe fn write_to(&self, port: &Port);
}

impl Output for u8 {
    unsafe fn write_to(&self, port: &Port) {
        outb(port.number, *self)
    }
}
