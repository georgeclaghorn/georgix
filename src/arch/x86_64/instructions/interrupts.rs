#[inline(always)]
pub unsafe fn sti() {
    asm!("sti", options(nomem, nostack));
}

#[inline(always)]
pub unsafe fn cli() {
    asm!("cli", options(nomem, nostack));
}

#[inline(always)]
pub unsafe fn lidt(pointer: &super::PointerDescriptor) {
    asm!("lidt [{}]", in(reg) pointer);
}
