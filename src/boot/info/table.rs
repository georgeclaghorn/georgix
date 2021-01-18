use super::tags::{Tags, Tag};

#[repr(C)]
pub struct Table {
    size: u32,
    _reserved: u32
}

impl Table {
    pub fn tags(&self) -> Tags {
        Tags::new(unsafe { self.as_ptr().offset(1) } as *const Tag)
    }

    fn as_ptr(&self) -> *const Table {
        self
    }
}
