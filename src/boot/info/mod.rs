mod table;
use table::Table;

mod tags;
use tags::Tags;

#[repr(C)]
pub struct Info {
    table: Table
}

impl Info {
    pub fn tags(&self) -> Tags {
        self.table.tags()
    }
}
