pub struct CS;

impl CS {
    pub fn get() -> u16 {
        let selector: u16;
        unsafe { asm!("mov {:x}, cs", out(reg) selector, options(nomem, nostack)); }
        selector
    }

    pub unsafe fn set(selector: u16) {
        asm!(
            "push {0:r}",
            "lea {0:r}, [rip + 1f]",
            "push {0:r}",
            "rex64 retf",
            "1:",
            inlateout(reg) selector => _,
            options(nomem)
        )
    }
}

pub struct SS;

impl SS {
    #[allow(dead_code)]
    pub fn get() -> u16 {
        let selector: u16;
        unsafe { asm!("mov {:x}, ss", out(reg) selector, options(nomem, nostack)); }
        selector
    }

    pub unsafe fn set(selector: u16) {
        asm!("mov ss, {:x}", in(reg) selector, options(nomem, nostack))
    }

    pub unsafe fn invalidate() {
        SS::set(0)
    }
}

pub struct TR;

impl TR {
    pub unsafe fn set(selector: u16) {
        crate::arch::x86_64::instructions::ltr(selector)
    }
}
