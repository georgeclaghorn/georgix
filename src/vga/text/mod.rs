mod colors;
pub mod console;

pub(self) use colors::{ColorCode, Color};

use volatile::Volatile;
use crate::arch::vga::registers::crtc::CURSOR_START_REGISTER;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Console {
    row: usize,
    column: usize,
    buffer: &'static mut Buffer
}

impl Console {
    fn new(buffer: &'static mut Buffer) -> Console {
        Console {
            row: 0,
            column: 0,
            buffer
        }
    }

    fn write_str(&mut self, string: &str, color: ColorCode) {
        for byte in string.bytes() {
            self.write_byte(byte, color);
        }
    }

    fn write_byte(&mut self, codepoint: u8, color: ColorCode) {
        match codepoint {
            b'\n' => self.new_line(),

            _ => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.put(Character::new(codepoint, color));
                self.advance();
            }
        }
    }

    fn new_line(&mut self) {
        if self.row == BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for column in 0..BUFFER_WIDTH {
                    let character = self.at(row, column).read();
                    self.at(row - 1, column).write(character);
                }
            }

            for column in 0..BUFFER_WIDTH {
                self.at(self.row, column).write(Character::blank());
            }
        } else {
            self.row += 1;
        }

        self.column = 0;
    }

    fn put(&mut self, character: Character) {
        self.current().write(character);
    }

    fn current(&mut self) -> &mut Volatile<Character> {
        self.at(self.row, self.column)
    }

    fn at(&mut self, row: usize, column: usize) -> &mut Volatile<Character> {
        &mut self.buffer.characters[row][column]
    }

    fn advance(&mut self) {
        self.column += 1;
    }

    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                self.at(row, column).write(Character::blank());
            }
        }

        CURSOR_START_REGISTER.lock().set(5);

        self.row = 0;
        self.column = 0;
    }
}

#[repr(transparent)]
struct Buffer {
    characters: [[Volatile<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Character {
    codepoint: u8,
    color: ColorCode
}

impl Character {
    fn new(codepoint: u8, color: ColorCode) -> Character {
        match codepoint {
            0x20..=0x7e => Character { codepoint, color },
            _ => Character { codepoint: 0xfe, color }
        }
    }

    fn blank() -> Character {
        Character {
            codepoint: 0,
            color: ColorCode::new(Color::Black, Color::Black)
        }
    }
}

impl PartialEq<char> for Character {
    fn eq(&self, other: &char) -> bool {
        char::from(self.codepoint) == *other
    }
}

impl PartialEq<Character> for char {
    fn eq(&self, other: &Character) -> bool {
        other == self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_a_character_with_an_invalid_codepoint() {
        assert_eq!(
            Character::new(195, ColorCode::new(Color::LightGray, Color::Black)),
            Character { codepoint: 0xfe, color: ColorCode::new(Color::LightGray, Color::Black) }
        );
    }
}
