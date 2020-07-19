mod interrupts;
pub use interrupts::*;

mod io;
pub use io::*;

mod misc;
pub use misc::*;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct PointerDescriptor {
    pub limit: u16,
    pub base: u64
}
