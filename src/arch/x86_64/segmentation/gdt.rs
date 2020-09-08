use crate::arch::x86_64::PrivilegeLevel;
use crate::arch::x86_64::instructions::{lgdt, Pointer};
use crate::arch::x86_64::multitasking::TaskStateSegment;

#[derive(Debug)]
#[repr(C)]
pub(super) struct GlobalDescriptorTable {
    entries: [u64; 8],
    next_available_index: usize
}

impl GlobalDescriptorTable {
    pub fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            entries: [0; 8],
            next_available_index: 1
        }
    }

    pub fn add(&mut self, descriptor: Descriptor) -> Selector {
        let index = match descriptor {
            Descriptor::UserSegment(value) => self.push(value),

            Descriptor::SystemSegment(low, high) => {
                let index = self.push(low);
                self.push(high);
                index
            }
        };

        let requested_privilege_level = match descriptor {
            Descriptor::UserSegment(value) => {
                if Flags::from_bits_truncate(value).contains(Flags::RING_3) {
                    PrivilegeLevel::Ring3
                } else {
                    PrivilegeLevel::Ring0
                }
            }

            Descriptor::SystemSegment(_, _) => PrivilegeLevel::Ring0
        };

        Selector::new(index as u16, requested_privilege_level)
    }

    fn push(&mut self, entry: u64) -> usize {
        if self.next_available_index < self.entries.len() {
            let index = self.next_available_index;
            self.entries[index] = entry;
            self.next_available_index += 1;
            index
        } else {
            panic!("GDT full")
        }
    }

    pub fn load(&self) {
        unsafe { lgdt(&Pointer::new(&self.entries)) }
    }
}


use bit_field::BitField;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(super) struct Selector(u16);

impl Selector {
    pub const fn new(index: u16, requested_privilege_level: PrivilegeLevel) -> Selector {
        Selector(index << 3 | (requested_privilege_level as u16))
    }

    pub fn index(self) -> u16 {
        self.0 >> 3
    }

    pub fn requested_privilege_level(self) -> PrivilegeLevel {
        PrivilegeLevel::from(self.0.get_bits(0..2))
    }
}

impl Into<u16> for Selector {
    fn into(self) -> u16 {
        self.0
    }
}

impl core::fmt::Debug for Selector {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.debug_struct("Selector")
            .field("index", &self.index())
            .field("requested_privilege_level", &self.requested_privilege_level())
            .finish()
    }
}


use bitflags::bitflags;

pub enum Descriptor {
    UserSegment(u64),
    SystemSegment(u64, u64)
}

impl Descriptor {
    pub fn kernel_code_segment() -> Descriptor {
        Descriptor::UserSegment(
            (
                Flags::USER_SEGMENT |
                Flags::PRESENT |
                Flags::EXECUTABLE |
                Flags::LONG_MODE
            ).bits()
        )
    }

    pub fn task_state_segment(segment: &'static TaskStateSegment) -> Descriptor {
        let address = segment as *const _ as u64;

        let mut low = Flags::PRESENT.bits();
        low.set_bits(16..40, address.get_bits(0..24));
        low.set_bits(56..64, address.get_bits(24..32));
        low.set_bits(0..16, (core::mem::size_of::<TaskStateSegment>() - 1) as u64);
        low.set_bits(40..44, 0b1001);

        let mut high = 0;
        high.set_bits(0..32, address.get_bits(32..64));

        Descriptor::SystemSegment(low, high)
    }
}

bitflags! {
    pub struct Flags: u64 {
        const WRITABLE     = 1 << 41;
        const CONFORMING   = 1 << 42;
        const EXECUTABLE   = 1 << 43;
        const USER_SEGMENT = 1 << 44;
        const PRESENT      = 1 << 47;
        const LONG_MODE    = 1 << 53;
        const RING_3       = 3 << 45;
    }
}
