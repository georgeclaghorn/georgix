#![cfg(test)]

#[no_mangle]
extern "Rust" fn __print(args: core::fmt::Arguments) {
    crate::arch::test::console::print(args);
}

#[no_mangle]
extern "Rust" fn __exit(code: u32) {
    crate::arch::test::exit(code);
}
