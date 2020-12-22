const SIGNATURE: &[u8; 8] = b"RSD PTR ";

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
        (0xE0000..0xFFFFF).step_by(16).find_map(|address| {
            let pointer = address as *const [u8; 8];

            unsafe {
                if *pointer == *SIGNATURE {
                    Some(*(pointer as *const RSDP))
                } else {
                    None
                }
            }
        })
    }
}
