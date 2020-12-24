#[inline(always)]
pub unsafe fn lgdt<T>(pointer: &super::Pointer<T>) {
    asm!("lgdt [{}]", in(reg) pointer);
}

#[inline(always)]
pub unsafe fn ltr(selector: u16) {
    asm!("ltr {:x}", in(reg) selector, options(nomem, nostack));
}
