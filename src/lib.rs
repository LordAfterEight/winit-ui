mod canvas;
pub mod color;
pub mod font;
pub mod app;

pub use app::App;
pub use font::Font;
pub use color::Color;

pub fn get_text_width(text: &str, size: f32, font: &Font) -> u32 {
    let mut width = 0;
    for char in text.chars() {
        let (metrics, _) = font.rasterize(char, size);
        width += metrics.advance_width as u32;
    }
    width
}