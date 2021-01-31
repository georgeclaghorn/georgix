#![no_std]
#![no_main]
#![feature(asm, global_asm)]
#![feature(abi_x86_interrupt)]

#![reexport_test_harness_main = "test"]

mod arch;
mod multiboot;
mod acpi;
mod vga;
mod util;
mod test;

use arch::park;

#[cfg(not(test))]
pub use vga::text::console;

#[cfg(test)]
pub use arch::test::console;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn main(magic: multiboot::Magic, info: &'static multiboot::Info) -> ! {
    // Initialize the console early for printing and panic handling.
    console::initialize();

    multiboot::magic::validate(magic);
    multiboot::info::set(info);

    println!("Georgix v{}", VERSION);

    if let Some(map) = info.memory_map() {
        print!("Memory map:\n{}", map);

        if let Some(address) = map.maximum_address() {
            println!("Maximum physical address: {:#x}", address);
        }
    } else {
        panic!("Memory map not found");
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
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}
