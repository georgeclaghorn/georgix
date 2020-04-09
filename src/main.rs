#![no_std]
#![no_main]
#![feature(asm, global_asm)]

mod arch;
mod boot;

use arch::*;

#[no_mangle]
fn main() -> ! {
    println!("Hello, {}!", "world");
    park();
}

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
