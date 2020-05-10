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
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GLOBAL_DESCRIPTOR_TABLE.0.load();

    unsafe {
        set_cs(GLOBAL_DESCRIPTOR_TABLE.1.code_selector);
        clear_ss();
        load_tss(GLOBAL_DESCRIPTOR_TABLE.1.task_state_segment_selector);
    }
}

unsafe fn clear_ss() {
    set_ss(0);
}

unsafe fn set_ss(value: u16) {
    llvm_asm!("movw $0, %ss" :: "r{bx}"(value));
}
