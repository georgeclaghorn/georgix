mod interrupts;
pub use interrupts::*;

mod io;
pub use io::*;

mod misc;
pub use misc::*;

#[repr(C, packed)]
pub struct PointerDescriptor<T> {
    limit: u16,
    base: *const T
}

impl<T> PointerDescriptor<T> {
    pub fn new(target: &T) -> PointerDescriptor<T> {
        PointerDescriptor {
            limit: core::mem::size_of::<T>() as u16,
            base: target as *const T
        }
    }
}
