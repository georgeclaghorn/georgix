#[inline(always)]
pub unsafe fn sti() {
    asm!("sti", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn cli() {
    asm!("cli", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn lidt<T>(pointer: &super::Pointer<T>) {
    asm!("lidt [{}]", in(reg) pointer)
}
