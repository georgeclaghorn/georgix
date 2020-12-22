use core::ops::Range;

const SIGNATURE: &[u8; 8] = b"RSD PTR ";
const LENGTH: usize = 24;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct RSDP {
    signature: [u8; 8],
    checksum:  u8,
    oem_id:    [u8; 6],
    revision:  u8,
    address:   usize
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
            let pointer = address as *const [u8; 8];

            if *pointer == *SIGNATURE {
                Some(*(pointer as *const RSDP))
            } else {
                None
            }
        })
    }

    fn validate(&self) -> bool {
        self.revision == 0 && self.sum() == 0
    }

    fn sum(&self) -> u8 {
        self.as_bytes().iter().fold(0, |sum, &byte| sum.wrapping_add(byte))
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const RSDP as *const u8, LENGTH) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validating_when_valid() {
        let rsdp =
            RSDP {
                signature: *b"RSD PTR ",
                checksum:  103,
                oem_id:    *b"BOCHS ",
                revision:  0,
                address:   0x7FE14D2
            };

        assert!(rsdp.validate());
    }

    #[test]
    fn validating_with_unsupported_revision() {
        let rsdp =
            RSDP {
                signature: *b"RSD PTR ",
                checksum:  102,
                oem_id:    *b"BOCHS ",
                revision:  1,
                address:   0x7FE14D2
            };

        assert!(!rsdp.validate());
    }

    #[test]
    fn validating_with_incorrect_checksum() {
        let rsdp =
            RSDP {
                signature: *b"RSD PTR ",
                checksum:  23,
                oem_id:    *b"BOCHS ",
                revision:  0,
                address:   0x7FE14D2
            };

        assert!(!rsdp.validate());
    }
}
