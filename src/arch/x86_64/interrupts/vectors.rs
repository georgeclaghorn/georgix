#[repr(u8)]
pub enum Vector {
    Timer = 32,
    Keyboard
}

impl Into<u8> for Vector {
    fn into(self) -> u8 {
        self as u8
    }
}
