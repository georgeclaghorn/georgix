#![cfg(target_arch = "x86_64")]

pub mod vga;
pub mod interrupts;

pub mod test;

mod boot;
mod instructions;
mod io;
mod segmentation;
mod util;

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
