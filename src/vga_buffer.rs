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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
struct ColorCode(u8);

impl ColorCode {
    fn new (foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // https://doc.rust-lang.org/nightly/nomicon/other-reprs.html#reprc
struct ScreenChar  {
    ascii_char: u8,
    color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

struct Writer {
    buffer: &'static mut Buffer,
    column_position: usize, // writer always write to the last row, therefore there's no row_position
    color_code: ColorCode
}

impl Writer {
    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(char);
            }
        }
        
        self.column_position = 0;
    }
    
    fn clear_row (&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code
        };
        
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row-1][col].write(blank);
        }
    }
    
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code
                });
                
                self.column_position += 1;
            }
        }
    }
    
    pub fn write_string (&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);

        Ok(())
    }
}

pub fn print_sth () {
    use core::fmt::Write;
    let mut writer = Writer {
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        color_code: ColorCode::new(Color::Blue, Color::White),
        column_position: 0
    };
    
    writer.write_string("Hello");
    writer.write_byte(b'!');
    writer.write_string("\n World");
    
    write!(&mut writer, "\n Numbers are {} and {}", 42, 1.0/3.0).unwrap();
}