use crate::colors::Colors;
use crate::font::VGA_FONT;
use crate::framebuffer::Framebuffer;
use core::fmt::{Arguments, Write};
use spin::Mutex;

pub static WRITER: Mutex<Option<TextDriver>> = Mutex::new(None);

pub fn init(framebuffer: Framebuffer) {
    let mut text_driver = TextDriver::new(framebuffer);
    text_driver.clear();
    *WRITER.lock() = Some(text_driver);
}

pub fn set_bg(bg: u32) {
    if let Some(text_driver) = WRITER.lock().as_mut() {
        text_driver.set_bg(bg);
    }
}

pub fn set_fg(fg: u32) {
    if let Some(text_driver) = WRITER.lock().as_mut() {
        text_driver.set_fg(fg);
    }
}

pub fn _print(args: Arguments) {
    if let Some(text_driver) = WRITER.lock().as_mut() {
        let _ = text_driver.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($a:tt)*) => {
        $crate::text_driver::_print(format_args!($($a)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($a:tt)*) => {
        $crate::print!("{}\n", format_args!($($a)*))
    };
}

// ~-no comment-~ //

pub struct TextDriver {
    framebuffer: Framebuffer,

    col: usize,
    row: usize,

    fg: u32,
    bg: u32,
}

impl TextDriver {
    pub fn new(framebuffer: Framebuffer) -> Self {
        TextDriver {
            framebuffer,
            col: 0,
            row: 0,
            fg: Colors::WHITE,
            bg: Colors::BLACK,
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(self.bg);
        self.col = 0;
        self.row = 0;
    }

    pub fn new_line(&mut self) {
        let h = VGA_FONT.char_size[1];
        let rows = self.framebuffer.height as usize / h;

        self.col = 0;
        self.row += 1;
        if self.row >= rows {
            self.scroll();
            self.row = rows - 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    pub fn scroll(&mut self) {
        self.framebuffer.scroll_up(VGA_FONT.char_size[1], self.bg);
    }

    pub fn draw_char(&mut self, c: char, cell_col: usize, cell_row: usize) {
        let w = VGA_FONT.char_size[0];
        let h = VGA_FONT.char_size[1];

        let mut code: usize = c as usize;
        if code >= 128 {
            // only ASCII, cause old DOS font
            code = b'?' as usize;
        }

        let start_offset: usize = code * h; // 8 bytes
        let x = (cell_col * w) as u64;
        let y = (cell_row * h) as u64;
        for row in 0..h {
            let byte = VGA_FONT.font_data[start_offset + row];
            for col in 0..w {
                let on = byte & (0x80 >> col) != 0;
                let color = if on { self.fg } else { self.bg };
                self.framebuffer
                    .put_pixel(x + col as u64, y + row as u64, color)
            }
        }
    }

    pub fn put_char(&mut self, c: char) {
        let w = VGA_FONT.char_size[0];

        if c == '\n' {
            self.new_line();
            return;
        } else if c == '\r' {
            self.col = 0;
            return;
        } else if c == '\t' {
            self.col = (self.col / 4 + 1) * 4; // moves to next 4 multiple
            if self.col >= self.framebuffer.width as usize / w {
                self.new_line();
            }
            return;
        } else if c == '\x08' {
            // backspace
            self.backspace();
            self.draw_char(' ', self.col, self.row);
            return;
        }

        self.draw_char(c, self.col, self.row);
        self.col += 1;
        if self.col >= self.framebuffer.width as usize / w {
            self.new_line();
        }
    }

    pub fn set_bg(&mut self, bg: u32) {
        self.bg = bg;
    }
    pub fn set_fg(&mut self, fg: u32) {
        self.fg = fg;
    }
}

impl Write for TextDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.put_char(c);
        }
        Ok(())
    }
}

unsafe impl Send for TextDriver {}
