use super::vectors::Vector;

#[repr(C)]
pub struct IOAPIC {
    index: volatile::WriteOnly<u32>,
    _1:    [u32; 3],
    data:  volatile::ReadWrite<u32>
}

impl IOAPIC {
    const BASE: u64 = 0xFEC00000;

    pub unsafe fn get() -> &'static mut IOAPIC {
        &mut *(IOAPIC::BASE as *mut _)
    }

    pub fn initialize(&mut self) {
        for index in 0..=23 {
            self.redirection_at(index).disable()
        }
    }

    pub fn enable(&mut self, index: u8, vector: Vector) {
        self.redirection_at(index).enable(vector)
    }

    fn redirection_at(&mut self, index: u8) -> Redirection {
        Redirection { owner: self, index }
    }

    fn register_at(&mut self, index: u8) -> Register {
        Register { owner: self, index }
    }

    fn read(&mut self, index: u8) -> u32 {
        self.index.write(index as u32);
        self.data.read()
    }

    fn write(&mut self, index: u8, data: u32) {
        self.index.write(index as u32);
        self.data.write(data);
    }
}

struct Redirection<'a> {
    owner: &'a mut IOAPIC,
    index: u8
}

impl<'a> Redirection<'a> {
    const BASE: u8 = 0x10;

    fn enable(&mut self, vector: Vector) {
        self.to_lower().write(vector as u32)
    }

    fn disable(&mut self) {
        self.to_lower().set(16);
        self.to_upper().write(0);
    }

    fn to_lower(&mut self) -> Register {
        self.owner.register_at(Redirection::BASE + 2 * self.index)
    }

    fn to_upper(&mut self) -> Register {
        self.owner.register_at(Redirection::BASE + 2 * self.index + 1)
    }
}

struct Register<'a> {
    owner: &'a mut IOAPIC,
    index: u8
}

impl<'a> Register<'a> {
    fn read(&mut self) -> u32 {
        self.owner.read(self.index)
    }

    fn write(&mut self, data: u32) {
        self.owner.write(self.index, data)
    }

    fn set(&mut self, bit: u8) {
        let value = self.read();
        self.write(value | (1 << bit))
    }
}
