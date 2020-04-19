#![cfg(test)]

use crate::println;
use crate::arch::test::*;

pub fn run(tests: &[&dyn Fn()]) {
    println!("Running {} tests:", tests.len());

    for test in tests {
        test();
    }

    exit(ExitCode::Success);
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit(ExitCode::Failure);
}
