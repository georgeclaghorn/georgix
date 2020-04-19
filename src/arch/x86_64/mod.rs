#![cfg(target_arch = "x86_64")]

pub mod boot;

pub fn park() -> ! {
    loop { halt(); }
}

pub fn halt() {
    unsafe { asm!("hlt"); }
}
