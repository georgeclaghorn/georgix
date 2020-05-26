#![no_std]
#![no_main]
#![feature(asm, global_asm)]
#![feature(abi_x86_interrupt)]

#![reexport_test_harness_main = "test"]

mod arch;
mod boot;
mod vga;
mod test;

use arch::*;

#[no_mangle]
pub extern "C" fn main(_magic: u32, _info: *const u8) -> ! {
    initialize();

    println!("Hello, {}!", "world");
    println!("Yeah, what's up, world?");

    #[cfg(test)]
    test();

    park();
}

fn initialize() {
    boot::console::initialize();
    arch::initialize();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    park();
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::boot::console::print(format_args!($($arg)*)));
}
