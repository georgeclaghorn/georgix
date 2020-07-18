#[inline(always)]
pub unsafe fn hlt() {
    asm!("hlt", options(nomem, nostack));
}

#[inline(always)]
pub unsafe fn rdmsrq(number: u32) -> u64 {
    let (high, low): (u64, u64);
    asm!("rdmsr", in("ecx") number, out("edx") high, out("eax") low, options(nomem, nostack));
    (high << 32) | low
}
