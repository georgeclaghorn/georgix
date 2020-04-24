#![no_std]
#![no_main]
#![feature(llvm_asm, global_asm)]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test::run)]
#![reexport_test_harness_main = "test"]

mod arch;
mod test;

use arch::*;

#[no_mangle]
pub extern "C" fn main(_magic: u32, _info: *const u8) -> ! {
    println!("Hello, {}!", "world");

    #[cfg(test)]
    test();

    park();
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
    ($($arg:tt)*) => ($crate::arch::boot::console::print(format_args!($($arg)*)));
}
