use bit_field::BitField;

use core::ops::Add;

#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    pub fn new(address: u64) -> VirtualAddress {
        VirtualAddress::try_new(address).expect("got non-canonical virtual address")
    }

    pub fn try_new(address: u64) -> Result<VirtualAddress, InvalidVirtualAddress> {
        match address.get_bits(47..64) {
            0 | 0x1ffff => Ok(VirtualAddress(address)),
            1 => Ok(VirtualAddress::truncate(address)),
            _ => Err(InvalidVirtualAddress(address))
        }
    }

    pub fn truncate(address: u64) -> VirtualAddress {
        VirtualAddress(((address << 16) as i64 >> 16) as u64)
    }

    pub const fn zero() -> VirtualAddress {
        VirtualAddress(0)
    }
}

impl<T> From<&T> for VirtualAddress {
    fn from(target: &T) -> VirtualAddress {
        VirtualAddress::from(target as *const T)
    }
}

impl<T> From<*const T> for VirtualAddress {
    fn from(target: *const T) -> VirtualAddress {
        VirtualAddress::new(target as u64)
    }
}

impl Add<u64> for VirtualAddress {
    type Output = Self;

    fn add(self, other: u64) -> Self::Output {
        VirtualAddress::new(self.0 + other)
    }
}

impl Add<usize> for VirtualAddress {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        self + other as u64
    }
}

impl core::fmt::Debug for VirtualAddress {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "VirtualAddress({:#x})", self.0)
    }
}

#[derive(PartialEq)]
pub struct InvalidVirtualAddress(u64);

impl core::fmt::Debug for InvalidVirtualAddress {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "InvalidVirtualAddress({:#x})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallibly_constructing_a_canonical_virtual_address_with_bit_47_cleared() {
        assert_eq!(
            VirtualAddress(0xfee00000),
            VirtualAddress::try_new(0xfee00000).expect("expected canonical VirtualAddress, got error")
        )
    }

    #[test]
    fn fallibly_constructing_a_canonical_virtual_address_with_bit_47_set() {
        assert_eq!(
            VirtualAddress(0xffff8000fee00000),
            VirtualAddress::try_new(0x8000fee00000).expect("expected canonical VirtualAddress, got error")
        )
    }

    #[test]
    fn fallibly_constructing_a_noncanonical_virtual_address() {
        assert_eq!(
            InvalidVirtualAddress(0xc00000fee00000),
            VirtualAddress::try_new(0xc00000fee00000).expect_err("expected InvalidVirtualAddress error")
        )
    }

    #[test]
    fn truncating_a_canonical_virtual_address_with_bit_47_cleared() {
        assert_eq!(VirtualAddress(0xfee00000), VirtualAddress::truncate(0xfee00000))
    }

    #[test]
    fn truncating_a_canonical_virtual_address_with_bit_47_set() {
        assert_eq!(VirtualAddress(0xffff8000fee00000), VirtualAddress::truncate(0x8000fee00000))
    }

    #[test]
    fn truncating_a_noncanonical_virtual_address_with_bit_47_cleared() {
        assert_eq!(VirtualAddress(0xfee00000), VirtualAddress::truncate(0xc00000fee00000))
    }

    #[test]
    fn truncating_a_noncanonical_virtual_address_with_bit_47_set() {
        assert_eq!(VirtualAddress(0xffff8000fee00000), VirtualAddress::truncate(0xc08000fee00000))
    }

    #[test]
    fn constructing_a_null_virtual_address() {
        assert_eq!(VirtualAddress(0), VirtualAddress::zero())
    }

    #[test]
    fn adding_to_a_virtual_address() {
        assert_eq!(VirtualAddress(0xfee00010), VirtualAddress(0xfee00000) + 0x10u64);
        assert_eq!(VirtualAddress(0xffff8000fee00000), VirtualAddress(0xfee00000) + (0x8000u64 << 32));

        assert_eq!(VirtualAddress(0xfee00010), VirtualAddress(0xfee00000) + 0x10usize);
        assert_eq!(VirtualAddress(0xffff8000fee00000), VirtualAddress(0xfee00000) + (0x8000usize << 32));
    }
}
