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

    fn redirection_at(&self, index: u8) -> Option<Redirection> {
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

    fn set(&self, bit: u8) {
        self.write(self.read() | (1 << bit))
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
            Some(Redirection { owner: self.owner, index })
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

struct Redirection<'a> {
    owner: &'a IOAPIC,
    index: u8
}

impl<'a> Redirection<'a> {
    const BASE: u8 = 0x10;

    fn enable(&self, vector: Vector) {
        self.to_lower().write(vector as u32)
    }

    fn disable(&self) {
        self.to_lower().set(16);
        self.to_upper().write(0);
    }

    fn to_lower(&self) -> Register {
        self.owner.register_at(Redirection::BASE + 2 * self.index)
    }

    fn to_upper(&self) -> Register {
        self.owner.register_at(Redirection::BASE + 2 * self.index + 1)
    }
}
