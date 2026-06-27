//! This module contains display related functions
//! and/or structs and enum's.

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
/// Define all the colors here with there color code.
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
    White = 15,
}

/// Implemented a new type, This will be used to pack both the
/// foreground and background color to one u8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Creates a new `ColorCode` struct with `higher` 4 bits used
    /// for storing color information about the background and the
    /// `lower` ones used for storing the foreground data.
    /// This operation is safe because the `color` enum only
    /// stores numbers upto `15` hence it only requires 4 bits
    /// of storage allowing us to efficiently use a single `u8`
    /// type to store both the foreground and background.
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
/// Stores the `ascii` character code with the color information for
/// the said character.
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// Define the hieght of the screen in terms of rows.
const BUFFER_HEIGHT: usize = 25;
/// Define the width of the screen in terms of columns.
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
/// Used for calculating the position of ascii character by the
/// `write` method on the `Writer` struct.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Used to get display output through vga.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    // Use `static` to  declare that the reference is valid throughout
    // the lifetime the program.
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Display one byte.
    pub fn write_byte(&mut self, byte: u8) {
        // Check if we recieved a "/n" to change line otherwise
        // display the byte.
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    /// Used for changing the line.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    /// Convert a string to printable ascii bytes.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
    // Clear a row by overwriting all of it's characters with blank spaces.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Declare a Global instance for display.
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
