use super::instructions::{inb, outb, inl, outl};

pub struct Port {
    number: u16
}

impl Port {
    pub fn new(number: u16) -> Port {
        Port { number }
    }

    pub unsafe fn read<T>(&self) -> T where T: Input {
        T::read_from(&self)
    }

    pub unsafe fn write<T>(&mut self, data: T) where T: Output {
        data.write_to(&self)
    }
}

pub trait Input {
    unsafe fn read_from(port: &Port) -> Self;
}

pub trait Output {
    unsafe fn write_to(&self, port: &Port);
}

impl Input for u8 {
    unsafe fn read_from(port: &Port) -> u8 {
        inb(port.number)
    }
}

impl Output for u8 {
    unsafe fn write_to(&self, port: &Port) {
        outb(port.number, *self)
    }
}

impl Input for u32 {
    unsafe fn read_from(port: &Port) -> u32 {
        inl(port.number)
    }
}

impl Output for u32 {
    unsafe fn write_to(&self, port: &Port) {
        outl(port.number, *self)
    }
}
