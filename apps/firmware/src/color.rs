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
        *self
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8, u8)) -> Self {
        Self {
            r: color.0,
            g: color.1,
            b: color.2,
            w: color.3,
        }
    }
}

impl Color {
    pub const fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            w: 0,
        }
    }

    #[allow(dead_code)]
    pub const fn new(r: u8, g: u8, b: u8, w: u8) -> Self {
        Self { r, g, b, w }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ()> {
        match hex::hex_to_rgbw(hex) {
            Ok((_, color)) => Ok(color),
            Err(_) => Err(()),
        }
    }

    pub const fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            w: 255,
        }
    }
}
