use core::fmt::Write;
use lazy_static::*;
use spin::Mutex;
use super::*;

lazy_static! {
    static ref CONSOLE: Mutex<Console> = Mutex::new(
        Console::new(unsafe { &mut *(0xb8000 as *mut Buffer) })
    );

    static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer {
            console: &CONSOLE,
            color: ColorCode::new(Color::LightGray, Color::Black)
        }
    );
}

pub fn initialize() {
    CONSOLE.lock().clear();
}

pub fn print(args: core::fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
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
    use crate::{print, println};
    use super::{print as print_graphically, CONSOLE};

    #[test_case]
    fn printing_one_line() {
        print!("vga::text::console::tests::printing_one_line... ");

        print_graphically(format_args!("Some test string that fits on a single line"));

        for (i, character) in "Some test string that fits on a single line".chars().enumerate() {
            let buffered_character = CONSOLE.lock().buffer.characters[0][i].read();
            assert_eq!(char::from(buffered_character.codepoint), character);
        }

        println!("[ok]");
    }

    #[test_case]
    fn printing_two_lines() {
        print!("vga::text::console::tests::printing_two_lines... ");

        print_graphically(format_args!("Some test string\nover two lines"));

        for (i, character) in "Some test string".chars().enumerate() {
            let buffered_character = CONSOLE.lock().buffer.characters[0][i].read();
            assert_eq!(char::from(buffered_character.codepoint), character);
        }

        for (i, character) in "over two lines".chars().enumerate() {
            let buffered_character = CONSOLE.lock().buffer.characters[1][i].read();
            assert_eq!(char::from(buffered_character.codepoint), character);
        }

        println!("[ok]");
    }
}
