#![cfg(target_arch = "x86_64")]

pub mod boot;
pub mod vga;

pub mod test;

mod interrupts;
mod segmentation;

pub fn initialize() {
    segmentation::initialize();
    interrupts::initialize();
}

#[inline(always)]
pub fn park() -> ! {
    loop { halt(); }
}

#[inline(always)]
pub fn halt() {
    unsafe { llvm_asm!("hlt" :::: "volatile"); }
}
