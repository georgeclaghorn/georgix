#![cfg(target_arch = "x86_64")]

global_asm!(include_str!("boot/header.S"));
global_asm!(include_str!("boot/start.S"));

pub fn park() -> ! {
    loop { halt(); }
}

pub fn halt() {
    unsafe { asm!("hlt"); }
}
