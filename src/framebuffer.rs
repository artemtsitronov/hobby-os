use core::ptr::copy;

pub struct Framebuffer {
    pub addr: *mut u8,
    pub pitch: u64,
    pub width: u64,
    pub height: u64,
}

impl Framebuffer {
    pub fn new(addr: *mut u8, pitch: u64, width: u64, height: u64) -> Self {
        Framebuffer {
            addr,
            pitch,
            width,
            height,
        }
    }

    pub fn put_pixel(&mut self, x: u64, y: u64, color: u32) {
        unsafe {
            let offset: u64 = y * self.pitch + x * 4;
            let pointer: *mut u32 = self.addr.add(offset as usize).cast::<u32>();
            pointer.write(color);
        }
    }

    pub fn fill(&mut self, color: u32) {
        self.draw_rect(0, 0, self.width, self.height, color);
    }

    pub fn draw_rect(&mut self, start_x: u64, start_y: u64, width: u64, height: u64, color: u32) {
        for y in start_y..height {
            for x in start_x..width {
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn scroll_up(&mut self, pixels: usize, fill: u32) {
        let shift = pixels * self.pitch as usize;
        let total = self.height as usize * self.pitch as usize;

        unsafe {
            copy(self.addr.add(shift), self.addr, total - shift);
        }

        self.draw_rect(
            0,
            self.height - pixels as u64,
            self.width,
            pixels as u64,
            fill,
        );
    }
}
