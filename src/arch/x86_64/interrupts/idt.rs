use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use x86_64::structures::idt::InterruptStackFrame;
use bit_field::BitField;
use bitflags::bitflags;
use super::Vector;

#[derive(Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: Entry<Handler>,
    pub debug: Entry<Handler>,
    pub non_maskable_interrupt: Entry<Handler>,
    pub breakpoint: Entry<Handler>,
    pub overflow: Entry<Handler>,
    pub bound_range_exceeded: Entry<Handler>,
    pub invalid_opcode: Entry<Handler>,
    pub device_not_available: Entry<Handler>,
    pub double_fault: Entry<DivergingHandlerWithErrorCode>,

    coprocessor_segment_overrun: Entry<Handler>,

    pub invalid_task_state_segment: Entry<HandlerWithErrorCode>,
    pub segment_not_present: Entry<HandlerWithErrorCode>,
    pub stack_segment_fault: Entry<HandlerWithErrorCode>,
    pub general_protection_fault: Entry<HandlerWithErrorCode>,
    pub page_fault: Entry<PageFaultHandler>,

    reserved_1: Entry<Handler>,

    pub x87_floating_point: Entry<Handler>,
    pub alignment_check: Entry<HandlerWithErrorCode>,
    pub machine_check: Entry<DivergingHandler>,
    pub simd_floating_point: Entry<Handler>,
    pub virtualization: Entry<Handler>,

    reserved_2: [Entry<Handler>; 9],

    pub security_exception: Entry<HandlerWithErrorCode>,

    reserved_3: Entry<Handler>,
    other: [Entry<Handler>; 256 - 32]
}

impl InterruptDescriptorTable {
    pub fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_error: Entry::new(),
            debug: Entry::new(),
            non_maskable_interrupt: Entry::new(),
            breakpoint: Entry::new(),
            overflow: Entry::new(),
            bound_range_exceeded: Entry::new(),
            invalid_opcode: Entry::new(),
            device_not_available: Entry::new(),
            double_fault: Entry::new(),

            coprocessor_segment_overrun: Entry::new(),

            invalid_task_state_segment: Entry::new(),
            segment_not_present: Entry::new(),
            stack_segment_fault: Entry::new(),
            general_protection_fault: Entry::new(),
            page_fault: Entry::new(),

            reserved_1: Entry::new(),

            x87_floating_point: Entry::new(),
            alignment_check: Entry::new(),
            machine_check: Entry::new(),
            simd_floating_point: Entry::new(),
            virtualization: Entry::new(),

            reserved_2: [Entry::new(); 9],

            security_exception: Entry::new(),

            reserved_3: Entry::new(),
            other: [Entry::new(); 256 - 32]
        }
    }

    pub fn load(&self) {
        unsafe {
            crate::arch::x86_64::instructions::lidt(
                &PointerDescriptor {
                    limit: (core::mem::size_of::<Self>() - 1) as u16,
                    base: self as *const _ as u64
                }
            )
        }
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = Entry<Handler>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0  => &self.divide_error,
            1  => &self.debug,
            2  => &self.non_maskable_interrupt,
            3  => &self.breakpoint,
            4  => &self.overflow,
            5  => &self.bound_range_exceeded,
            6  => &self.invalid_opcode,
            7  => &self.device_not_available,
            9  => &self.coprocessor_segment_overrun,
            16 => &self.x87_floating_point,
            19 => &self.simd_floating_point,
            20 => &self.virtualization,

            i @ 8 | i @ 10..=14 | i @ 17 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            },

            i @ 15 | i @ 21..=29 | i @ 31 => panic!("entry {} is reserved", i),

            i @ 18 => panic!("entry {} is a diverging exception", i),

            i @ 32..=255 => &self.other[i - 32],

            i => panic!("no entry with index {}", i)
        }
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0  => &mut self.divide_error,
            1  => &mut self.debug,
            2  => &mut self.non_maskable_interrupt,
            3  => &mut self.breakpoint,
            4  => &mut self.overflow,
            5  => &mut self.bound_range_exceeded,
            6  => &mut self.invalid_opcode,
            7  => &mut self.device_not_available,
            9  => &mut self.coprocessor_segment_overrun,
            16 => &mut self.x87_floating_point,

            i @ 8 | i @ 10..=14 | i @ 17 | i @ 30 => {
                panic!("entry {} is an exception with error code", i)
            },

            i @ 15 | i @ 21..=29 | i @ 31 => panic!("entry {} is reserved", i),

            i @ 18 => panic!("entry {} is a diverging exception", i),

            i @ 32..=255 => &mut self.other[i - 32],

            i => panic!("no entry with index {}", i)
        }
    }
}

impl Index<Vector> for InterruptDescriptorTable {
    type Output = Entry<Handler>;

    fn index(&self, index: Vector) -> &Self::Output {
        &self[index.into(): usize]
    }
}

impl IndexMut<Vector> for InterruptDescriptorTable {
    fn index_mut(&mut self, index: Vector) -> &mut Self::Output {
        &mut self[index.into(): usize]
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Entry<H> {
    pointer_low: u16,
    code_segment_selector: u16,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<H>
}

impl<H> Entry<H> {
    pub fn new() -> Entry<H> {
        Entry {
            code_segment_selector: x86_64::instructions::segmentation::cs().0,
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
            phantom: PhantomData
        }
    }

    fn handle_at_address(&mut self, address: u64) -> &mut EntryOptions {
        self.point_to(address);
        self.present(true)
    }

    fn point_to(&mut self, address: u64) {
        self.pointer_low    = address as u16;
        self.pointer_middle = (address >> 16) as u16;
        self.pointer_high   = (address >> 32) as u32;
    }

    fn present(&mut self, value: bool) -> &mut EntryOptions {
        self.options.present(value)
    }
}

impl Entry<Handler> {
    pub fn handle_with(&mut self, handler: Handler) -> &mut EntryOptions {
        self.handle_at_address(handler as u64)
    }
}

impl Entry<HandlerWithErrorCode> {
    pub fn handle_with(&mut self, handler: HandlerWithErrorCode) -> &mut EntryOptions {
        self.handle_at_address(handler as u64)
    }
}

impl Entry<DivergingHandlerWithErrorCode> {
    pub fn handle_with(&mut self, handler: DivergingHandlerWithErrorCode) -> &mut EntryOptions {
        self.handle_at_address(handler as u64)
    }
}

impl Entry<PageFaultHandler> {
    pub fn handle_with(&mut self, handler: PageFaultHandler) -> &mut EntryOptions {
        self.handle_at_address(handler as u64)
    }
}


#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntryOptions(u16);

impl EntryOptions {
    pub fn minimal() -> EntryOptions {
        EntryOptions(0b1110_0000_0000)
    }

    pub fn present(&mut self, value: bool) -> &mut EntryOptions {
        self.0.set_bit(15, value);
        self
    }

    pub unsafe fn on_stack_with_index(&mut self, index: u16) -> &mut EntryOptions {
        self.0.set_bits(0..3, index + 1);
        self
    }
}


pub type Handler = extern "x86-interrupt" fn (&mut InterruptStackFrame);
pub type HandlerWithErrorCode = extern "x86-interrupt" fn (&mut InterruptStackFrame, u64);

pub type DivergingHandler = extern "x86-interrupt" fn (&mut InterruptStackFrame) -> !;
pub type DivergingHandlerWithErrorCode = extern "x86-interrupt" fn (&mut InterruptStackFrame, u64) -> !;

pub type PageFaultHandler = extern "x86-interrupt" fn (&mut InterruptStackFrame, PageFaultErrorCode);


bitflags! {
    #[repr(transparent)]
    pub struct PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1;
        const CAUSED_BY_WRITE      = 1 << 1;
        const USER_MODE            = 1 << 2;
        const MALFORMED_TABLE      = 1 << 3;
        const INSTRUCTION_FETCH    = 1 << 4;
    }
}


#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct PointerDescriptor {
    limit: u16,
    base: u64
}


#[cfg(test)]
mod test {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn sizing() {
        assert_eq!(size_of::<Entry<Handler>>(), 16);
        assert_eq!(size_of::<InterruptDescriptorTable>(), 256 * 16);
    }
}
