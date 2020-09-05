#[inline(always)]
pub unsafe fn lgdt<T>(pointer: &super::Pointer<T>) {
    asm!("lgdt [{}]", in(reg) pointer);
}
