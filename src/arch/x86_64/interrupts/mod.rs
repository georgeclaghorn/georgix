mod handlers;

mod pic;
use pic::{ChainedPIC, PIC};

mod apic;
use apic::APIC;

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
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

        table[Vector::Timer.into()].set_handler_fn(self::handlers::timer);

        table
    };

    static ref PICS: Mutex<ChainedPIC> = Mutex::new(
        ChainedPIC::new(
            PIC::new(0x20, 0x21),
            PIC::new(0xA0, 0xA1)
        )
    );

    static ref LAPIC: Mutex<&'static mut APIC> = Mutex::new(APIC::get());
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

fn complete() {
    LAPIC.lock().complete();
}

#[repr(u8)]
enum Vector {
    Timer = 32,
    SpuriousInterrupt = 63
}

impl Into<usize> for Vector {
    fn into(self) -> usize {
        self as usize
    }
}

impl core::ops::BitOr<Vector> for u32 {
    type Output = u32;

    fn bitor(self, vector: Vector) -> u32 {
        self | vector as u32
    }
}
