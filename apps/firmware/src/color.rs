use crate::utils::hex;

#[derive(Debug, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub w: u8,
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            w: self.w,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.r = source.r;
        self.g = source.g;
        self.b = source.b;
        self.w = source.w;
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            w: 0,
        }
    }
}

impl Into<Color> for (u8, u8, u8, u8) {
    fn into(self: (u8, u8, u8, u8)) -> Color {
        Color {
            r: self.0,
            g: self.1,
            b: self.2,
            w: self.3,
        }
    }
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8, w: u8) -> Self {
        Self { r, g, b, w }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ()> {
         match  hex::hex_to_rgbw(hex) {
Ok((_, color)) -> 
            Ok(color),
            Err(_) -> Err(())        }
    }

    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            w: 255,
        }
    }
}
