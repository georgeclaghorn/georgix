mod info;
pub use info::Info;

#[cfg(not(test))]
pub use crate::vga::text::console;

#[cfg(test)]
pub use crate::arch::test::console;
