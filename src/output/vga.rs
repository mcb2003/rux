use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref BUFFER: Mutex<Writer> = Mutex::new(unsafe { Writer::new() });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    BUFFER.lock().write_fmt(args).unwrap();
}
/// A VGA color.
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

/// A VGA foreground and background color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Create a new ColorCode from a foreground and background color.
    pub fn new(fg: Color, bg: Color) -> Self {
        Self((bg as u8) << 4 | fg as u8)
    }
}

/// A single character with associated color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color: ColorCode,
}

impl ScreenChar {
    /// Construct a new `ScreenChar` with the specified character and `ColorCode`.
    pub fn new(character: u8, color: ColorCode) -> Self {
        Self { character, color }
    }
}

/// Represents the VGA text buffer (found at `0xb8000`)
#[repr(transparent)]
struct Buffer([[ScreenChar; Buffer::WIDTH]; Buffer::HEIGHT]);

impl Buffer {
    /// Width (in characters) of the VGA text buffer
    pub const WIDTH: usize = 80;
    /// Height (in characters) of the VGA text buffer
    pub const HEIGHT: usize = 25;

    /// Get a mutable raw pointer to a location on the screen.
    ///
    /// # Panics
    ///
    /// * If `row` or `col` are out of bounds.
    fn mut_ptr_at(&mut self, row: usize, col: usize) -> *mut ScreenChar {
        &mut self.0[row][col] as _
    }

    /// Clear a row of characters.
    ///
    /// # Panics
    ///
    /// * If the row is out of bounds.
    pub fn clear_row(&mut self, row: usize, color: ColorCode) {
        let blank = ScreenChar::new(b'\0', color);
        for c in 0..Buffer::WIDTH {
            let ptr = self.mut_ptr_at(row, c);
            unsafe {
                ptr.write_volatile(blank);
            }
        }
    }

    /// Clear the entire screen
    pub fn clear_screen(&mut self, color: ColorCode) {
        let blank = ScreenChar::new(b'\0', color);
        for r in 0..Buffer::HEIGHT {
        for c in 0..Buffer::WIDTH {
            let ptr = self.mut_ptr_at(r, c);
            unsafe {
                ptr.write_volatile(blank);
            }
        }
        }
    }
}

pub struct Writer {
    row: usize,
    col: usize,
    pub color: ColorCode,
    buff: &'static mut Buffer,
}

impl Writer {
    /// Create a new writer that points to the VGA buffer.
    ///
    /// # Safety
    ///
    /// The code must have access to the VGA text buffer at `0xb8000`.
    unsafe fn new() -> Self {
            let color = ColorCode::new(Color::White, Color::Black);
        let buff = &mut *(0xb8000 as *mut Buffer);
        buff.clear_screen(color);
        Self {
            row: 0,
            col: 0,
            color,
            buff,
        }
    }

    /// Move all lines up by one, and scroll the top line off the screen.
    fn scroll_down(&mut self) {
        for r in 1..Buffer::HEIGHT {
            for c in 0..Buffer::WIDTH {
                let ptr = self.buff.mut_ptr_at(r - 1, c);
                unsafe {
                    ptr.write_volatile(self.buff.0[r][c]);
                }
            }
        }
        self.buff.clear_row(Buffer::HEIGHT - 1, self.color);
    }

    /// Start a new line on the screen, scrolling if necessary.
    fn new_line(&mut self) {
        self.col = 0;
        if self.row < Buffer::HEIGHT - 1 {
            self.row += 1;
        } else {
            self.scroll_down();
        }
    }

    /// Write a single byte to the screen at the current cursor position, with the current
    /// `ColorCode`.
    pub fn write_byte(&mut self, c: u8) {
        match c {
            b'\n' => self.new_line(),
            c => {
                let ptr = self.buff.mut_ptr_at(self.row, self.col);
                unsafe {
                    ptr.write_volatile(ScreenChar::new(c, self.color));
                }
                self.col += 1; // Imcrement cursor position
                if self.col >= Buffer::WIDTH {
                    self.new_line();
                }
            }
        }
    }

    /// Write a sequence of bytes starting at the current cursor position, with the current
    /// `ColorCode`.
    pub fn write_bytes(&mut self, bytes: impl AsRef<[u8]>) {
        for &byte in bytes.as_ref() {
            match byte {
                // printable
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not printable
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s);
        Ok(())
    }
}
