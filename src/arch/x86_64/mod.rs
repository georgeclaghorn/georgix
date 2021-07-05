#![cfg(target_arch = "x86_64")]

pub mod vga;
pub mod memory;
pub mod interrupts;

pub mod test;

mod instructions;
use instructions::flags;

mod boot;
mod io;
mod multitasking;
mod registers;

use crate::acpi;

pub fn initialize() {
    memory::initialize();
    acpi::initialize();
    interrupts::initialize();
    interrupts::enable();
}

#[inline(always)]
pub fn park() -> ! {
    loop { halt() }
}

#[inline(always)]
pub fn halt() {
    unsafe { self::instructions::hlt() }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3
}

impl From<u16> for PrivilegeLevel {
    fn from(value: u16) -> PrivilegeLevel {
        match value {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            _ => panic!("{} is not a valid privilege level", value)
        }
    }
}
