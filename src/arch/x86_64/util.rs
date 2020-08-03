#[inline(always)]
pub(super) fn rflags() -> u64 {
    let rflags: u64;

    unsafe {
        asm!("pushf", "pop {}", out(reg) rflags, options(nomem));
    }

    rflags
}
