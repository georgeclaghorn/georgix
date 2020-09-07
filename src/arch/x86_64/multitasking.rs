use super::addresses::VirtualAddress;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TaskStateSegment {
    reserved_1: u32,
    pub privilege_stack_table: [VirtualAddress; 3],
    reserved_2: u64,
    pub interrupt_stack_table: [VirtualAddress; 7],
    reserved_3: u64,
    reserved_4: u16,
    pub iomap_base: u16
}

impl TaskStateSegment {
    pub const fn new() -> TaskStateSegment {
        TaskStateSegment {
            privilege_stack_table: [VirtualAddress::zero(); 3],
            interrupt_stack_table: [VirtualAddress::zero(); 7],
            iomap_base: 0,

            reserved_1: 0,
            reserved_2: 0,
            reserved_3: 0,
            reserved_4: 0
        }
    }
}
