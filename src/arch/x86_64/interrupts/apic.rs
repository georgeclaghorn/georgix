pub struct APIC {
    task_priority_register: Register,
    end_of_interrupt_register: Register,
    spurious_interrupt_vector_register: Register,
    timer_vector_register: Register,
    timer_initial_count_register: Register,
    timer_divide_configuration_register: Register
}

impl APIC {
    pub fn new(base: u64) -> APIC {
        APIC {
            task_priority_register: Register::new(base + 0x0080),
            end_of_interrupt_register: Register::new(base + 0x00B0),
            spurious_interrupt_vector_register: Register::new(base + 0x00F0),
            timer_vector_register: Register::new(base + 0x0320),
            timer_initial_count_register: Register::new(base + 0x380),
            timer_divide_configuration_register: Register::new(base + 0x03E0)
        }
    }

    pub fn initialize(&self) {
        unsafe {
            self.spurious_interrupt_vector_register.write(0x00000100 | (32 + 31));

            self.timer_vector_register.write(0x00020000 | (32 + 0));
            self.timer_initial_count_register.write(10000000);
            self.timer_divide_configuration_register.write(0xB);

            self.end_of_interrupt_register.write(0);

            self.task_priority_register.write(0);
        }
    }

    pub fn complete(&self) {
        unsafe { self.end_of_interrupt_register.write(0); }
    }
}

struct Register {
    address: u64
}

impl Register {
    fn new(address: u64) -> Register {
        Register { address }
    }

    unsafe fn write(&self, value: u32) {
        self.to_mut_ptr().write(value)
    }

    fn to_mut_ptr(&self) -> *mut u32 {
        self.address as *mut u32
    }
}
