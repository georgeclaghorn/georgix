#[repr(C)]
#[derive(Debug)]
pub struct Tag {
    pub kind: Kind,
    pub size: u32
}

#[repr(u32)]
#[derive(PartialEq, Debug)]
pub enum Kind {
    End = 0,
    MemoryMap = 6
}

use core::convert::TryFrom;

macro_rules! try_from_impl_for {
    ($name:ident, $kind:expr) => {
        impl<'a> TryFrom<&'a Tag> for &'a $name {
            type Error = ();

            fn try_from(tag: &'a Tag) -> Result<&'a $name, ()> {
                if tag.kind == $kind {
                    Ok(unsafe { &*(tag as *const Tag as *const $name) })
                } else {
                    Err(())
                }
            }
        }
    }
}

use super::memory::Region;

#[repr(C)]
#[derive(Debug)]
pub struct MemoryMapTag {
    pub kind: Kind,
    pub size: u32,
    pub entry_size: u32,
    _entry_version: u32
}

impl MemoryMapTag {
    pub(super) fn first_region(&self) -> *const Region {
        unsafe { self.as_ptr().offset(1) as *const Region }
    }

    pub(super) fn last_region(&self) -> *const Region {
        (self.as_ptr() as u32 + self.size - self.entry_size) as *const Region
    }

    fn as_ptr(&self) -> *const MemoryMapTag {
        self
    }
}

try_from_impl_for!(MemoryMapTag, Kind::MemoryMap);
