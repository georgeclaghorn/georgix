#![no_std]
#![no_main]
#![feature(asm, global_asm)]
#![feature(abi_x86_interrupt)]

#![reexport_test_harness_main = "test"]

mod arch;
mod boot;
mod acpi;
mod vga;
mod test;

use arch::park;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn main() -> ! {
    boot::console::initialize();

    println!("Georgix v{}", VERSION);

    arch::initialize();

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
    ($($arg:tt)*) => ($crate::boot::console::print(format_args!($($arg)*)));
}
