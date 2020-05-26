#[inline(always)]
pub unsafe fn outb(port: u16, data: u8) {
    asm!("out dx, al", in("dx") port, in("al") data, options(nomem));
}

#[inline(always)]
pub unsafe fn sti() {
    asm!("sti", options(nomem, nostack));
}

#[inline(always)]
pub unsafe fn cli() {
    asm!("cli", options(nomem, nostack));
}

#[inline(always)]
pub unsafe fn hlt() {
    asm!("hlt", options(nomem, nostack));
}
