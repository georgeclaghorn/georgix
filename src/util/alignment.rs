pub fn align_up(address: usize, alignment: usize) -> usize {
    let mask = alignment - 1;

    if address & mask == 0 {
        address
    } else {
        (address + mask) & !mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligning_an_already_aligned_address_up() {
        assert_eq!(0xFEE00000, align_up(0xFEE00000, 8))
    }

    #[test]
    fn aligning_an_unaligned_address_up() {
        assert_eq!(0xFEE00008, align_up(0xFEE00004, 8))
    }
}
