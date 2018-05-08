use core::fmt;

use spin::Mutex;
use volatile::Volatile;

use io::{Io, Port};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        current_line: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
    static ref VGA_COMMAND: Port<u8> = Port::new(0x3D4);
    static ref VGA_DATA: Port<u8> = Port::new(0x3D5);
}

/// Encapsulates writing to the VGA buffer
pub struct Writer {
    column_position: usize,
    current_line: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_cell(&mut self, row: usize, column: usize, screen_char: ScreenChar) {
        self.buffer.chars[row][column].write(screen_char);
    }

    /// Writes a single byte to the VGA buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.column_position = 0,
            127 => self.backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.current_line;
                let col = self.column_position;

                let color_code = self.color_code;
                self.write_cell(
                    row,
                    col,
                    ScreenChar {
                        ascii_char: byte,
                        color_code,
                    },
                );
                // Set the color of the next space so the scanline is visible
                if col <= BUFFER_WIDTH - 2 {
                    self.clear_cell(row, col + 1);
                }
                self.column_position += 1;
            }
        }
        Self::move_cursor(self.column_position as u16, self.current_line as u16);
    }

    /// Writes an entire string to the VGA buffer
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    /// Advance writer
    fn new_line(&mut self) {
        let current_line = self.current_line;
        // Check if the next line will be past the screen
        if current_line + 1 > BUFFER_HEIGHT - 1 {
            // Shift all characters up one row
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(current_line);
        } else {
            self.current_line += 1;
        }
        self.column_position = 0;
        // Set the color of the next space so the scanline is visible
        let (row, col) = (self.current_line, self.column_position);
        self.clear_cell(row, col);
    }

    /// Delete the last character
    fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
        }
        let (row, col) = (self.current_line, self.column_position);
        self.clear_cell(row, col);
    }

    #[cfg(not(test))]
    pub fn move_cursor(x: u16, y: u16) {
        let pos = y * BUFFER_WIDTH as u16 + x;
        VGA_COMMAND.write(0x0F);
        VGA_DATA.write((pos & 0xFF) as u8);

        VGA_COMMAND.write(0x0E);
        VGA_DATA.write(((pos >> 8) & 0xFF) as u8);
    }

    #[cfg(test)]
    pub fn move_cursor(_x: u16, _y: u16) {}

    /// Clears a single row of the VGA buffer
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.write_cell(row, col, blank);
        }
    }

    /// Clears a single cell of the VGA buffer
    fn clear_cell(&mut self, row: usize, column: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        self.write_cell(row, column, blank);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// VGA Color Codes
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

/// A full color code, represents forground and background colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    /// Creates a new ColorCode from forground and background colors
    const fn new(forground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | forground as u8)
    }
}

/// Represents a character on the screen, contains both the ascii ordinal and ColorCode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

/// VGA Text screen buffer
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Writes arguments to VGA buffer from print[ln] macros
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == 0 && j == 0 {
                    assert_eq!(screen_char.ascii_char, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == 0 && j == 1 {
                    assert_eq!(screen_char.ascii_char, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn write_formatted() {
        use core::fmt::Write;
        use std::io::Write as IoWrite;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == 0 && j == 0 {
                    assert_eq!(screen_char.ascii_char, b'a');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == 1 && j == 0 {
                    assert_eq!(screen_char.ascii_char, b'b');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == 1 && j == 1 {
                    assert_eq!(screen_char.ascii_char, b'c');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i >= 1 {
                    assert_eq!(screen_char.ascii_char, b' ');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            column_position: 0,
            current_line: 0,
            color_code: ColorCode::new(Color::Green, Color::Black),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;
        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_char: b' ',
            color_code: ColorCode::new(Color::Green, Color::Black),
        }
    }
}
