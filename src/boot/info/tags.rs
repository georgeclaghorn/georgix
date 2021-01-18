use core::marker::PhantomData;

pub struct Tags<'a> {
    current: *const Tag,
    phantom: PhantomData<&'a Tag>
}

impl<'a> Tags<'a> {
    pub fn new(first: *const Tag) -> Tags<'a> {
        Tags {
            current: first,
            phantom: PhantomData
        }
    }

    unsafe fn current(&self) -> Option<&'a Tag> {
        match &*self.current {
            Tag { kind: 0, size: _ } => None,
            tag => Some(tag)
        }
    }
}

use tap::tap::Tap;
use crate::util::alignment::align_up;

impl<'a> Iterator for Tags<'a> {
    type Item = &'a Tag;

    fn next(&mut self) -> Option<&'a Tag> {
        unsafe { self.current() }.tap(|tag| {
            if let Some(tag) = tag {
                self.current = align_up(self.current as usize + tag.size as usize, 8) as *const Tag
            }
        })
    }
}

#[repr(C)]
pub struct Tag {
    pub kind: u32,
    pub size: u32
}
