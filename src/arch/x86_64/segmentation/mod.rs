mod gdt;
use gdt::{GlobalDescriptorTable, Selector, Descriptor};

use super::multitasking::TaskStateSegment;
use super::addresses::VirtualAddress;
use super::registers::segmentation::*;

use lazy_static::lazy_static;

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;

lazy_static! {
    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, Selectors) = {
        let mut table = GlobalDescriptorTable::new();
        let code_selector = table.add(Descriptor::kernel_code_segment());
        let task_state_segment_selector = table.add(Descriptor::task_state_segment(&TASK_STATE_SEGMENT));
        (table, Selectors { code_selector, task_state_segment_selector })
    };

    static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut task_state_segment = TaskStateSegment::new();

        task_state_segment.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            const STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            VirtualAddress::from(&STACK) + STACK_SIZE
        };

        task_state_segment
    };
}

struct Selectors {
    code_selector: Selector,
    task_state_segment_selector: Selector
}

pub fn initialize() {
    GLOBAL_DESCRIPTOR_TABLE.0.load();

    unsafe {
        CS::set(GLOBAL_DESCRIPTOR_TABLE.1.code_selector.into());
        SS::invalidate();
        TR::set(GLOBAL_DESCRIPTOR_TABLE.1.task_state_segment_selector.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalidating_the_stack_segment_selector_on_boot() {
        assert_eq!(0, SS::get())
    }
}
