#[repr(transparent)]
pub struct VirtualAddress(u64);

impl core::fmt::Debug for VirtualAddress {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "VirtualAddress({:#x})", self.0)
    }
}
