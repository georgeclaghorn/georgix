pub mod handlers;

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use pic8259_simple::ChainedPics;
use spin::Mutex;

use super::util::rflags;

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut table = InterruptDescriptorTable::new();

        table.breakpoint.set_handler_fn(self::handlers::breakpoint);

        unsafe {
            table.double_fault.set_handler_fn(self::handlers::double_fault)
                .set_stack_index(super::segmentation::DOUBLE_FAULT_STACK_INDEX);
        }

        table.general_protection_fault.set_handler_fn(self::handlers::general_protection_fault);
        table.page_fault.set_handler_fn(self::handlers::page_fault);

        table[Index::Timer.as_usize()].set_handler_fn(self::handlers::timer);

        table
    };
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub(super) fn initialize() {
    INTERRUPT_DESCRIPTOR_TABLE.load();

    unsafe {
        PICS.lock().initialize();
    }
}

pub(super) fn enable() {
    unsafe { super::instructions::sti(); }
}

pub(super) fn disable() {
    unsafe { super::instructions::cli(); }
}

pub(super) fn enabled() -> bool {
    (rflags() & 0x200) != 0
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

fn end(index: Index) {
    unsafe { PICS.lock().notify_end_of_interrupt(index.as_u8()) }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Index {
    Timer = PIC_1_OFFSET
}

impl Index {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
