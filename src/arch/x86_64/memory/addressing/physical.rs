use bit_field::BitField;

use core::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    pub fn new(address: u64) -> PhysicalAddress {
        PhysicalAddress::try_new(address).expect("got non-canonical physical address")
    }

    pub fn try_new(address: u64) -> Result<PhysicalAddress, ()> {
        match address.get_bits(52..64) {
            0 => Ok(PhysicalAddress(address)),
            _ => Err(())
        }
    }
}

impl Add<u64> for PhysicalAddress {
    type Output = Self;

    fn add(self, other: u64) -> Self::Output {
        PhysicalAddress::new(self.0 + other)
    }
}

impl Add<usize> for PhysicalAddress {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        self + other as u64
    }
}

impl core::fmt::LowerHex for PhysicalAddress {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(formatter)
    }
}
