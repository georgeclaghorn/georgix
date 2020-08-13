#[repr(usize)]
pub enum Vector {
    Timer = 32
}

impl Into<usize> for Vector {
    fn into(self) -> usize {
        self as usize
    }
}

impl core::ops::BitOr<Vector> for u32 {
    type Output = u32;

    fn bitor(self, vector: Vector) -> u32 {
        self | vector as u32
    }
}
