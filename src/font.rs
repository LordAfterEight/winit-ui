pub struct Font {
    font: fontdue::Font,
}

impl Font {
    pub fn new(path: &str) -> Self {
        let font_data = std::fs::read(path).unwrap();
        let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();
        Self { font }
    }

    pub fn rasterize(&self, char: char, size: f32) -> (fontdue::Metrics, Vec<u8>) {
        self.font.rasterize(char, size)
    }
}