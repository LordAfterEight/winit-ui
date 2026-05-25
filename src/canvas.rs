pub struct Canvas<'a> {
    frame: &'a mut [u8],
    pub width: u32,
    pub height: u32,
}

impl<'a> Canvas<'a> {
    pub(crate) fn new(frame: &'a mut [u8], width: u32, height: u32) -> Self {
        Self { frame, width, height }
    }

    fn idx(&self, x: u32, y: u32) -> usize {
        ((y * self.width + x) * 4) as usize
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: &crate::color::Color) {
        if x >= self.width || y >= self.height { return; }
        let i = self.idx(x, y);
        self.frame[i]     = color.r;
        self.frame[i + 1] = color.g;
        self.frame[i + 2] = color.b;
        self.frame[i + 3] = color.a;
    }

    pub fn clear(&mut self, color: crate::color::Color) {
        for pixel in self.frame.chunks_exact_mut(4) {
            pixel[0] = color.r;
            pixel[1] = color.g;
            pixel[2] = color.b;
            pixel[3] = color.a;
        }
    }

    pub fn draw_rect_f(&mut self, x: u32, y: u32, w: u32, h: u32, color: &crate::color::Color) {
        for row in y..(y + h) {
            for col in x..(x + w) {
                self.set_pixel(col, row, color);
            }
        }
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: &crate::color::Color) {
        for col in x..(x + w) {
            self.set_pixel(col, y, color);
            self.set_pixel(col, y + h - 1, color);
        }
        for row in y..(y + h) {
            self.set_pixel(x, row, color);
            self.set_pixel(x + w - 1, row, color);
        }
    }

    pub fn draw_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: &crate::color::Color) {
        let mut x0 = x0 as i32;
        let mut y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.set_pixel(x0 as u32, y0 as u32, &color);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx; }
            if e2 <= dx { err += dx; y0 += sy; }
        }
    }

    pub fn draw_circle_f(&mut self, cx: u32, cy: u32, radius: u32, color: &crate::color::Color) {
        let cx = cx as i32;
        let cy = cy as i32;
        let radius = radius as i32;

        for y in (cy - radius)..=(cy + radius) {
            for x in (cx - radius)..=(cx + radius) {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= radius * radius {
                    if x >= 0 && y >= 0 {
                        self.set_pixel(x as u32, y as u32, color);
                    }
                }
            }
        }
    }

    pub fn draw_circle(&mut self, cx: u32, cy: u32, radius: u32, color: &crate::color::Color) {
        let cx = cx as i32;
        let cy = cy as i32;
        let radius = radius as i32;

        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.set_pixel((cx + x) as u32, (cy + y) as u32, color);
            self.set_pixel((cx + y) as u32, (cy + x) as u32, color);
            self.set_pixel((cx - y) as u32, (cy + x) as u32, color);
            self.set_pixel((cx - x) as u32, (cy + y) as u32, color);
            self.set_pixel((cx - x) as u32, (cy - y) as u32, color);
            self.set_pixel((cx - y) as u32, (cy - x) as u32, color);
            self.set_pixel((cx + y) as u32, (cy - x) as u32, color);
            self.set_pixel((cx + x) as u32, (cy - y) as u32, color);

            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    pub fn draw_text(&mut self, mut x: u32, y: u32, size: f32, text: &str, font: &crate::font::Font, color: &crate::color::Color) {
        for char in text.chars() {
            if char.is_ascii() {
                let (metrics, glyph) = font.rasterize(char, size);
                for gx in 0..metrics.width {
                    for gy in 0..metrics.height {
                        let alpha = glyph[gy * metrics.width + gx];
                        let blended_color = crate::color::Color {
                            r: ((color.r as u16 * alpha as u16) / 255) as u8,
                            g: ((color.g as u16 * alpha as u16) / 255) as u8,
                            b: ((color.b as u16 * alpha as u16) / 255) as u8,
                            a: alpha,
                        };
                        let px = (x as i32 + gx as i32 + metrics.xmin) as u32;
                        let py = (y as i32 - metrics.height as i32 + gy as i32 - metrics.ymin) as u32;
                        self.set_pixel(px, py, &blended_color);
                    }
                }
                x += metrics.advance_width as u32;
            }
        }
    }
}