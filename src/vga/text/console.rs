use core::fmt::Write;
use lazy_static::*;
use spin::Mutex;
use super::*;

lazy_static! {
    static ref CONSOLE: Mutex<Console> = Mutex::new(
        Console::new(unsafe { &mut *(0xB8000 as *mut Buffer) })
    );

    static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer {
            console: &CONSOLE,
            color: ColorCode::new(Color::LightGray, Color::Black)
        }
    );
}

pub fn initialize() {
    CONSOLE.lock().clear()
}

pub fn print(args: core::fmt::Arguments) {
    crate::arch::interrupts::suppress(|| WRITER.lock().write_fmt(args).unwrap())
}

struct Writer {
    console: &'static Mutex<Console>,
    color: ColorCode
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        self.console.lock().write_str(string, self.color);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::vga::registers::crtc::CURSOR_START_REGISTER;

    #[test]
    fn printing_one_line() {
        initialize();
        print(format_args!("Some test string that fits on a single line"));

        for (i, expected) in "Some test string that fits on a single line".chars().enumerate() {
            assert_eq!(expected, character_at(0, i));
        }
    }

    #[test]
    fn printing_one_line_that_wraps() {
        initialize();

        print(
            format_args!(
                "'The time has come,' the Walrus said, 'to talk of many things: \
                of shoes, and ships, and sealing wax, and cabbages and kings, \
                and why the sea is boiling hot, and whether pigs have wings.'"
            )
        );

        for (i, expected) in "'The time has come,' the Walrus said, 'to talk of many things: of shoes, and shi".chars().enumerate() {
            assert_eq!(expected, character_at(0, i));
        }

        for (i, expected) in "ps, and sealing wax, and cabbages and kings, and why the sea is boiling hot, and".chars().enumerate() {
            assert_eq!(expected, character_at(1, i));
        }

        for (i, expected) in " whether pigs have wings.'".chars().enumerate() {
            assert_eq!(expected, character_at(2, i));
        }

        for i in 26..80 {
            assert_eq!(Character::blank(), character_at(2, i));
        }
    }

    #[test]
    fn printing_two_lines() {
        initialize();
        print(format_args!("Some test string\nover two lines"));

        for (i, expected) in "Some test string".chars().enumerate() {
            assert_eq!(expected, character_at(0, i));
        }

        for (i, expected) in "over two lines".chars().enumerate() {
            assert_eq!(expected, character_at(1, i));
        }
    }

    #[test]
    fn printing_many_lines() {
        initialize();

        for i in 1..=100 {
            print(format_args!("Hello, {}!\n", i));
        }

        for (i, expected) in "Hello, 77!".chars().enumerate() {
            assert_eq!(expected, character_at(0, i));
        }

        for (i, expected) in "Hello, 90!".chars().enumerate() {
            assert_eq!(expected, character_at(13, i));
        }

        for (i, expected) in "Hello, 100!".chars().enumerate() {
            assert_eq!(expected, character_at(23, i));
        }

        for i in 0..80 {
            assert_eq!(Character::blank(), character_at(24, i));
        }
    }

    #[test]
    fn disabling_the_cursor_on_initialize() {
        initialize();
        assert!(CURSOR_START_REGISTER.lock().get(5));
    }

    fn character_at(row: usize, column: usize) -> Character {
        CONSOLE.lock().buffer.characters[row][column].read()
    }
}
