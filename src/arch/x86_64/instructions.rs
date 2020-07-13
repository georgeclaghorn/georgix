#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack));
    value
}

#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let value: u32;
    asm!("in eax, dx", in("dx") port, out("eax") value, options(nomem, nostack));
    value
}

#[inline(always)]
pub unsafe fn outl(port: u16, value: u32) {
    asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack));
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

#[inline(always)]
pub unsafe fn rdmsrq(number: u32) -> u64 {
    let (high, low): (u64, u64);
    asm!("rdmsr", in("ecx") number, out("edx") high, out("eax") low, options(nomem, nostack));
    (high << 32) | low
}
