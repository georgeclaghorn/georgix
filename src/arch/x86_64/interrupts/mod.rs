pub mod handlers;

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut table = InterruptDescriptorTable::new();

        table.breakpoint.set_handler_fn(self::handlers::breakpoint);

        unsafe {
            table.double_fault.set_handler_fn(self::handlers::double_fault)
                .set_stack_index(super::segmentation::DOUBLE_FAULT_STACK_INDEX);
        }

        table.general_protection_fault.set_handler_fn(self::handlers::general_protection_fault);

        table
    };
}

pub fn initialize() {
    INTERRUPT_DESCRIPTOR_TABLE.load();
}
