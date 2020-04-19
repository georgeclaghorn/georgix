#![cfg(test)]

use crate::println;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
enum ExitCode {
    Success = 0x10,
    Failure = 0x11
}

use x86_64::instructions::port::Port;

fn exit(code: ExitCode) -> ! {
    unsafe {
        Port::new(0xF4).write(code as u32);
        core::hint::unreachable_unchecked();
    }
}
