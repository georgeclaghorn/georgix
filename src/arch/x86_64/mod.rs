#![cfg(target_arch = "x86_64")]

pub mod boot;
pub mod test;
pub mod vga;

pub fn park() -> ! {
    loop { halt(); }
}

pub fn halt() {
    unsafe { llvm_asm!("hlt" :::: "volatile"); }
}
