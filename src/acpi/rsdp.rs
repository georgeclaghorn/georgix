use core::ops::Range;
use core::convert::TryInto;

const SIGNATURE: &[u8; 8] = b"RSD PTR ";

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct RSDP {
    signature:    [u8; 8],
    _1:           u8,
    oem_id:       [u8; 6],
    revision:     u8,
    rsdt_address: u32,

    // These fields are only available for revision > 0.
    length:       u32,
    xsdt_address: u64,
    _2:           u8,
    _3:           [u8; 3]
}

impl RSDP {
    pub fn find() -> Option<RSDP> {
        // Search the BIOS ROM.
        RSDP::find_in(0xE0000..0xFFFFF)
    }

    fn find_in(area: Range<usize>) -> Option<RSDP> {
        // This is safe because we validate the resulting RSDP.
        unsafe { RSDP::scan(area) }.filter(|rsdp| rsdp.validate())
    }

    unsafe fn scan(area: Range<usize>) -> Option<RSDP> {
        area.step_by(16).find_map(|address| {
            let signature = address as *const [u8; 8];

            if *signature == *SIGNATURE {
                Some(*(address as *const RSDP))
            } else {
                None
            }
        })
    }

    pub fn address(&self) -> Option<usize> {
        if self.revision == 0 {
            self.rsdt_address.try_into()
        } else {
            self.xsdt_address.try_into()
        }.ok()
    }

    fn validate(&self) -> bool {
        self.checksum() == 0
    }

    fn checksum(&self) -> u8 {
        self.as_bytes().iter().fold(0, |sum, &byte| sum.wrapping_add(byte))
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, self.length()) }
    }

    fn length(&self) -> usize {
        if self.revision == 0 {
            20
        } else {
            self.length as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validating_a_v1_rsdp_when_valid() {
        let rsdp =
            RSDP {
                signature:    *b"RSD PTR ",
                _1:           103,
                oem_id:       *b"BOCHS ",
                revision:     0,
                rsdt_address: 0x7FE14D2,

                length:       0,
                xsdt_address: 0,
                _2:           0,
                _3:           [0; 3]
            };

        assert!(rsdp.validate());
    }

    #[test]
    fn validating_a_v1_rsdp_when_invalid() {
        let rsdp =
            RSDP {
                signature:    *b"RSD PTR ",
                _1:           23,
                oem_id:       *b"BOCHS ",
                revision:     0,
                rsdt_address: 0x7FE14D2,

                length:       0,
                xsdt_address: 0,
                _2:           0,
                _3:           [0; 3]
            };

        assert!(!rsdp.validate());
    }

    #[test]
    fn getting_the_rsdt_address_from_a_v1_rsdp() {
        let rsdp =
            RSDP {
                signature:    *b"RSD PTR ",
                _1:           103,
                oem_id:       *b"BOCHS ",
                revision:     0,
                rsdt_address: 0x7FE14D2,

                length:       0,
                xsdt_address: 0,
                _2:           0,
                _3:           [0; 3]
            };

        assert_eq!(Some(0x7FE14D2), rsdp.address());
    }
}
