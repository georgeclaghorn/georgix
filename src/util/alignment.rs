pub fn align_up(address: usize, alignment: usize) -> usize {
    let mask = alignment - 1;

    if address & mask == 0 {
        address
    } else {
        (address + mask) & !mask
    }
}
