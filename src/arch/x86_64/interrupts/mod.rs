pub mod handlers;

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use pic8259_simple::ChainedPics;
use spin::Mutex;

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

    x86_64::instructions::interrupts::enable();
}

pub fn without<F, R>(f: F) -> R
where
    F: FnOnce() -> R
{
    x86_64::instructions::interrupts::without_interrupts(f)
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
