use volatile::Volatile;
use super::Vector;

#[repr(C)]
pub struct APIC {
    _unused_1: [u32; 32],
    task_priority_register: Volatile<u32>,
    _unused_2: [u32; 11],
    end_of_interrupt_register: Volatile<u32>,
    _unused_3: [u32; 15],
    spurious_interrupt_vector_register: Volatile<u32>,
    _unused_4: [u32; 139],
    timer_vector_register: Volatile<u32>,
    _unused_5: [u32; 23],
    timer_initial_count_register: Volatile<u32>,
    _unused_6: [u32; 23],
    timer_divide_configuration_register: Volatile<u32>
}

impl APIC {
    pub fn get() -> &'static mut APIC {
        // TODO: Read the LAPIC's base address from the IA32_APIC_BASE MSR.
        unsafe { &mut *(0xFEE00000 as *mut APIC) }
    }

    pub fn initialize(&mut self) {
        self.spurious_interrupt_vector_register.write(0x00000100 | Vector::SpuriousInterrupt);

        self.timer_vector_register.write(0x00020000 | Vector::Timer);
        self.timer_initial_count_register.write(10000000);
        self.timer_divide_configuration_register.write(0xB);
        self.end_of_interrupt_register.write(0);

        self.task_priority_register.write(0);
    }

    pub fn complete(&mut self) {
        self.end_of_interrupt_register.write(0);
    }
}
