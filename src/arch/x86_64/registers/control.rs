use crate::arch::x86_64::memory::VirtualAddress;

// Page fault linear address
pub struct CR2;

impl CR2 {
    pub fn read() -> VirtualAddress {
        let value: u64;
        unsafe { asm!("mov {:r}, cr2", out(reg) value, options(nomem, nostack)); }
        VirtualAddress::new(value)
    }
}
