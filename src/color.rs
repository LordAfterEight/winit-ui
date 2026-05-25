pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a 
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let value = value.trim_start_matches('#');
        let r = u8::from_str_radix(&value[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&value[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&value[4..6], 16).unwrap_or(0);
        Self { r, g, b, a: 255 }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            r: (value >> 16) as u8,
            g: (value >> 8) as u8,
            b: value as u8,
            a: 255,
        }
    }
}