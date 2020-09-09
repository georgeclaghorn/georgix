mod vectors;
use vectors::Vector;

mod handlers;

mod idt;
use idt::InterruptDescriptorTable;

mod pic;
use pic::{ChainedPIC, PIC};

mod apic;
use apic::APIC;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut table = InterruptDescriptorTable::new();

        table.breakpoint.handle_with(self::handlers::breakpoint);

        unsafe {
            table.double_fault.handle_with(self::handlers::double_fault)
                .on_stack_with_index(super::segmentation::DOUBLE_FAULT_STACK_INDEX);
        }

        table.general_protection_fault.handle_with(self::handlers::general_protection_fault);
        table.page_fault.handle_with(self::handlers::page_fault);

        table[Vector::Timer].handle_with(self::handlers::timer);

        table
    };

    static ref PICS: Mutex<ChainedPIC> = Mutex::new(
        ChainedPIC::new(
            PIC::new(0x20, 0x21),
            PIC::new(0xA0, 0xA1)
        )
    );

    static ref LAPIC: Mutex<&'static mut APIC> = Mutex::new(unsafe { APIC::get() });
}

pub(super) fn initialize() {
    INTERRUPT_DESCRIPTOR_TABLE.load();
    PICS.lock().disable();
    LAPIC.lock().initialize();
}

pub(super) fn enable() {
    unsafe { super::instructions::sti(); }
}

pub(super) fn disable() {
    unsafe { super::instructions::cli(); }
}

pub(super) fn enabled() -> bool {
    (super::flags() & 0x200) != 0
}

pub fn suppress<F, R>(f: F) -> R where F: FnOnce() -> R {
    let enabled = enabled();

    if enabled {
        disable();
    }

    let result = f();

    if enabled {
        enable();
    }

    result
}

fn acknowledge() {
    LAPIC.lock().acknowledge();
}
