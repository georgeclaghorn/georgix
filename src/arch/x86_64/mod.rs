#![cfg(target_arch = "x86_64")]

pub mod vga;
pub mod interrupts;

pub mod test;

mod instructions;
use instructions::flags;

mod addresses;
mod boot;
mod io;
mod segmentation;

pub fn initialize() {
    segmentation::initialize();
    interrupts::initialize();
    interrupts::enable();
}

#[inline(always)]
pub fn park() -> ! {
    loop { halt(); }
}

#[inline(always)]
pub fn halt() {
    unsafe { self::instructions::hlt(); }
}
