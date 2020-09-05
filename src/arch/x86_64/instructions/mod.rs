mod interrupts;
pub use interrupts::*;

mod io;
pub use io::*;

mod segmentation;
pub use segmentation::*;

mod misc;
pub use misc::*;

#[repr(C, packed)]
pub struct Pointer<T> {
    limit: u16,
    base: *const T
}

impl<T> Pointer<T> {
    pub fn new(target: &T) -> Pointer<T> {
        Pointer {
            limit: core::mem::size_of::<T>() as u16,
            base: target as *const T
        }
    }
}
