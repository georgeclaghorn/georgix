#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_a_light_gray_on_black_color_code() {
        assert_eq!(
            ColorCode::new(Color::LightGray, Color::Black),
            ColorCode(7)
        )
    }

    #[test]
    fn constructing_a_white_on_green_color_code() {
        assert_eq!(
            ColorCode::new(Color::White, Color::Green),
            ColorCode(47)
        )
    }
}
