use bit_field::BitField;

use core::ops::Add;

#[derive(Clone, Copy)]
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
            other => Err(InvalidVirtualAddress(other))
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

impl <T> From<*const T> for VirtualAddress {
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

pub struct InvalidVirtualAddress(u64);

impl core::fmt::Debug for InvalidVirtualAddress {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "InvalidVirtualAddress({:#x})", self.0)
    }
}
