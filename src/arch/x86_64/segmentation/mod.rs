mod gdt;
use gdt::{GlobalDescriptorTable, Selector, Descriptor};

use super::multitasking::TaskStateSegment;
use super::addresses::VirtualAddress;

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
        set_code_segment_selector(GLOBAL_DESCRIPTOR_TABLE.1.code_selector.into());
        invalidate_stack_segment_selector();
        set_task_state_segment_selector(GLOBAL_DESCRIPTOR_TABLE.1.task_state_segment_selector.into());
    }
}

pub fn get_code_segment_selector() -> u16 {
    let selector: u16;
    unsafe { asm!("mov {:x}, cs", out(reg) selector, options(nomem, nostack)); }
    selector
}

unsafe fn set_code_segment_selector(value: u16) {
    asm!(
        "push {0:r}",
        "lea {0:r}, [rip + 1f]",
        "push {0:r}",
        "rex64 retf",
        "1:",
        inlateout(reg) value => _,
        options(nomem)
    )
}

#[allow(dead_code)]
fn get_stack_segment_selector() -> u16 {
    let selector: u16;
    unsafe { asm!("mov {:x}, ss", out(reg) selector, options(nomem, nostack)); }
    selector
}

unsafe fn invalidate_stack_segment_selector() {
    set_stack_segment_selector(0)
}

unsafe fn set_stack_segment_selector(value: u16) {
    asm!("mov ss, {:x}", in(reg) value, options(nomem, nostack))
}

unsafe fn set_task_state_segment_selector(value: u16) {
    asm!("ltr {:x}", in(reg) value, options(nomem, nostack))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalidating_the_stack_segment_selector_on_boot() {
        assert_eq!(0, get_stack_segment_selector())
    }
}
