#![no_std]
#![feature(fn_traits, unboxed_closures)]

mod integration;
mod types;

use integration::{print, exit};
pub use types::*;

pub fn test_main_static(tests: &[&TestDescAndFn]) {
    println!("Running {} tests:", tests.len());

    for test in tests {
        print!("{}... ", test.desc.name);
        (test.testfn)();
        println!("[ok]");
    }

    exit(0);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit(1);
}

pub fn assert_test_result<T: Termination>(result: T) {
    assert_eq!(result.report(), 0);
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print(format_args!($($arg)*)));
}
