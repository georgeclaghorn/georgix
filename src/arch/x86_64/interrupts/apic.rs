use super::Vector;

const IA32_APIC_BASE_MSR: u32 = 0x1B;

#[repr(C)]
pub struct APIC {
    _1: [u32; 44],
    end_of_interrupt_register: volatile::WriteOnly<u32>,
    _2: [u32; 155],
    timer_vector_register: volatile::ReadWrite<u32>,
    _3: [u32; 23],
    timer_initial_count_register: volatile::ReadWrite<u32>,
    _4: [u32; 23],
    timer_divide_configuration_register: volatile::ReadWrite<u32>
}

impl APIC {
    // This function is unsafe because two separate callers will get mutable references to the same
    // underlying data. It is the caller’s responsibility to ensure no other code writes to the
    // APIC concurrently.
    pub unsafe fn get() -> &'static mut APIC {
        &mut *(APIC::base() as *mut APIC)
    }

    fn base() -> u64 {
        // This is safe because the IA32_APIC_BASE MSR is architecture-specified.
        unsafe { crate::arch::x86_64::instructions::rdmsrq(IA32_APIC_BASE_MSR) & 0xFFFFFF000 }
    }

    pub fn initialize(&mut self) {
        self.timer_vector_register.write(0x20000 | Vector::Timer);
        self.timer_initial_count_register.write(10000000);
        self.timer_divide_configuration_register.write(0xB);
        self.end_of_interrupt_register.write(0);
    }

    pub fn acknowledge(&mut self) {
        self.end_of_interrupt_register.write(0)
    }
}
