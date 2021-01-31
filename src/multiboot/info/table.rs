use super::tags::{Tag, Kind};

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

use core::marker::PhantomData;
use core::convert::TryInto;
use crate::util::alignment::align_up;

pub struct Tags<'a> {
    current: *const Tag,
    phantom: PhantomData<&'a Tag>
}

impl<'a> Tags<'a> {
    fn new(first: *const Tag) -> Tags<'a> {
        Tags {
            current: first,
            phantom: PhantomData
        }
    }

    pub fn get<T>(&mut self, kind: Kind) -> Option<&'a T> where &'a Tag: TryInto<&'a T> {
        self.find(|tag| tag.kind == kind).and_then(|tag| tag.try_into().ok())
    }

    unsafe fn current(&self) -> Option<&'a Tag> {
        match &*self.current {
            Tag { kind: Kind::End, size: _ } => None,
            tag => Some(tag)
        }
    }

    fn advance(&self, offset: usize) -> *const Tag {
        align_up(self.current as usize + offset, 8) as *const Tag
    }
}

use tap::tap::Tap;

impl<'a> Iterator for Tags<'a> {
    type Item = &'a Tag;

    fn next(&mut self) -> Option<&'a Tag> {
        unsafe { self.current() }.tap(|tag| {
            if let Some(tag) = tag {
                self.current = self.advance(tag.size as usize)
            }
        })
    }
}
