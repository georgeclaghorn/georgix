#![no_std]
#![no_main]
#![feature(asm, global_asm)]
#![feature(abi_x86_interrupt)]

#![reexport_test_harness_main = "test"]

mod arch;
mod boot;
mod acpi;
mod vga;
mod util;
mod test;

use arch::park;
use boot::Info;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn main(magic: u32, info: &Info) -> ! {
    boot::console::initialize();

    if magic != 0x36D76289 {
        panic!("Georgix requires a Multiboot 2-compliant bootloader");
    }

    println!("Georgix v{}", VERSION);

    for tag in info.tags() {
        println!("0x{:x}\tTag: {}, 0x{:x}", tag as *const _ as usize, tag.kind, tag.size);
    }

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
