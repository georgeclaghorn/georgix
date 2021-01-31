pub type Magic = u32;

const EXPECTED: Magic = 0x36D76289;

pub fn validate(actual: Magic) {
    if actual != EXPECTED {
        panic!("Georgix requires a Multiboot 2-compliant bootloader");
    }
}
