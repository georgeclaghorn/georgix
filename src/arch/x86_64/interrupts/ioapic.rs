use spin::Mutex;
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
        for index in 0..=23 {
            self.redirection_at(index).disable()
        }
    }

    pub fn enable(&self, index: u8, vector: Vector) {
        self.redirection_at(index).enable(vector)
    }

    fn redirection_at(&self, index: u8) -> Redirection {
        Redirection { owner: self, index }
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
