#[allow(improper_ctypes)]
extern "Rust" {
    fn __print(args: core::fmt::Arguments);
    fn __exit(status: u32) -> !;
}

pub fn print(args: core::fmt::Arguments) {
    unsafe { __print(args) }
}

pub fn exit(code: u32) -> ! {
    unsafe { __exit(code) }
}
