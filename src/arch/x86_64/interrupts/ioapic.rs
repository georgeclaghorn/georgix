#![allow(dead_code)]

use spin::Mutex;
use bit_field::BitField;
use tap::tap::Tap;

use super::vectors::Vector;

pub struct IOAPIC {
    registers: Mutex<&'static mut Registers>
}

impl IOAPIC {
    const BASE: u64 = 0xFEC00000;

    pub unsafe fn get() -> IOAPIC {
        IOAPIC { registers: Mutex::new(&mut *(IOAPIC::BASE as *mut _)) }
    }

    pub fn initialize(&self) {
        for redirection in self.redirections() {
            redirection.disable()
        }
    }

    pub fn enable(&self, index: u8, vector: Vector) {
        if let Some(redirection) = self.redirection_at(index) {
            redirection.enable(vector)
        }
    }

    fn redirections(&self) -> Redirections {
        Redirections::new(self)
    }

    pub fn redirection_at(&self, index: u8) -> Option<Redirection> {
        self.redirections().get(index)
    }

    fn register_at(&self, index: u8) -> Register {
        Register { owner: self, index }
    }

    fn read(&self, index: u8) -> u32 {
        self.registers.lock().read(index)
    }

    fn write(&self, index: u8, data: u32) {
        self.registers.lock().write(index, data)
    }
}

#[repr(C)]
struct Registers {
    index: volatile::WriteOnly<u32>,
    _1:    [u32; 3],
    data:  volatile::ReadWrite<u32>
}

impl Registers {
    fn read(&mut self, index: u8) -> u32 {
        self.index.write(index as u32);
        self.data.read()
    }

    fn write(&mut self, index: u8, data: u32) {
        self.index.write(index as u32);
        self.data.write(data);
    }
}

struct Register<'a> {
    owner: &'a IOAPIC,
    index: u8
}

impl<'a> Register<'a> {
    fn read(&self) -> u32 {
        self.owner.read(self.index)
    }

    fn write(&self, data: u32) {
        self.owner.write(self.index, data)
    }

    fn get(&self, index: u8) -> bool {
        self.read().get_bit(index.into())
    }

    fn set(&self, index: u8, value: bool) {
        self.write(*self.read().set_bit(index.into(), value))
    }
}

struct Redirections<'a> {
    owner: &'a IOAPIC,
    count: u8,
    index: u8
}

impl<'a> Redirections<'a> {
    fn new(owner: &'a IOAPIC) -> Redirections {
        Redirections {
            owner,
            count: owner.read(0x01).get_bits(16..=23) as u8,
            index: 0
        }
    }

    fn get(&self, index: u8) -> Option<Redirection<'a>> {
        if index < self.count {
            Some(Redirection::new(self.owner, index))
        } else {
            None
        }
    }
}

impl<'a> Iterator for Redirections<'a> {
    type Item = Redirection<'a>;

    fn next(&mut self) -> Option<Redirection<'a>> {
        self.get(self.index).tap(|redirection| {
            if redirection.is_some() {
                self.index += 1
            }
        })
    }
}

pub struct Redirection<'a> {
    lower: Register<'a>,
    upper: Register<'a>
}

impl<'a> Redirection<'a> {
    const BASE: u8 = 0x10;

    fn new(owner: &'a IOAPIC, index: u8) -> Redirection<'a> {
        Redirection {
            lower: owner.register_at(Redirection::BASE + 2 * index),
            upper: owner.register_at(Redirection::BASE + 2 * index + 1)
        }
    }

    fn enable(&self, vector: Vector) {
        self.lower.write(vector as u32)
    }

    fn disable(&self) {
        self.lower.set(16, true);
        self.upper.write(0);
    }

    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }

    pub fn is_disabled(&self) -> bool {
        self.lower.get(16)
    }

    pub fn vector(&self) -> u8 {
        self.lower.read() as u8
    }
}
