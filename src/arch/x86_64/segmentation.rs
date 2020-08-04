use lazy_static::lazy_static;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;

lazy_static! {
    static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };

    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, Selectors) = {
        let mut table = GlobalDescriptorTable::new();
        let code_selector = table.add_entry(Descriptor::kernel_code_segment());
        let task_state_segment_selector = table.add_entry(Descriptor::tss_segment(&TASK_STATE_SEGMENT));
        (table, Selectors { code_selector, task_state_segment_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    task_state_segment_selector: SegmentSelector
}

pub fn initialize() {
    GLOBAL_DESCRIPTOR_TABLE.0.load();

    unsafe {
        set_code_segment_selector(GLOBAL_DESCRIPTOR_TABLE.1.code_selector.0);
        set_stack_segment_selector(0);
        set_task_state_segment_selector(GLOBAL_DESCRIPTOR_TABLE.1.task_state_segment_selector.0);
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

unsafe fn set_stack_segment_selector(value: u16) {
    asm!("mov ss, {:x}", in(reg) value, options(nomem, nostack))
}

unsafe fn set_task_state_segment_selector(value: u16) {
    asm!("ltr {:x}", in(reg) value, options(nomem, nostack))
}

#[cfg(test)]
mod tests {
    #[test]
    fn clearing_ss() {
        let ss;
        unsafe { asm!("mov {:x}, ss", out(reg) ss); }
        assert_eq!(0, ss);
    }
}
