#[inline(always)]
pub unsafe fn outb(port: u16, byte: u8) {
    asm!("out dx, al", in("dx") port, in("al") byte, options(nomem, nostack));
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
