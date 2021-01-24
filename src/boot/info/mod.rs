mod table;
use table::{Table, Tags};

mod tags;
use tags::*;

mod mem;
pub use mem::MemoryMap;

use spin::RwLock;

static INFO: RwLock<Option<&'static Info>> = RwLock::new(None);

pub fn set(info: &'static Info) {
    INFO.write().replace(info);
}

pub fn get() -> Option<&'static Info> {
    *INFO.read()
}

#[repr(C)]
pub struct Info {
    table: Table
}

impl Info {
    pub fn memory_map(&self) -> Option<MemoryMap> {
        self.tags().get(Kind::MemoryMap).map(|tag: &MemoryMapTag| tag.into())
    }

    fn tags(&self) -> Tags {
        self.table.tags()
    }
}
