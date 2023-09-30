use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::primitives::triangle;
use smart_leds::RGB8;

pub trait NextFrame {
    fn next_frame(&mut self) -> &RGB8;
}

pub trait currrent_frame {
    fn current_frame(&self) -> &RGB8;
}

struct RainbowAnimation {
    current_color: RGB8,
    pub is_static: bool,
}

pub struct StaticColorAnimation {
    colors: Vec<RGB8>,
    current_index: usize,
    pub is_static: bool,
}

impl StaticColorAnimation {
    pub fn new(colors: Vec<RGB8>) -> Self {
        Self {
            colors,
            current_index: 0,
            is_static: true,
        }
    }
}

impl RainbowAnimation {
    pub fn new() -> Self {
        Self {
            current_color: RGB8::new(255, 255, 255),
            is_static: false,
        }
    }
}

impl NextFrame for StaticColorAnimation {
    fn next_frame(&mut self) -> &RGB8 {
        self.current_index += 1;
        if self.current_index >= self.colors.len() - 1 {
            self.current_index = 0;
        }

        return self.colors.get(self.current_index).unwrap();
    }
}

impl currrent_frame for StaticColorAnimation {
    fn current_frame(&self) -> &RGB8 {
        return self.colors.get(self.current_index).unwrap();
    }
}

impl NextFrame for RainbowAnimation {
    fn next_frame(&self) -> &RGB8 {
        return &self.current_color;
    }
}

impl currrent_frame for RainbowAnimation {
    fn current_frame(&self) -> &RGB8 {
        return &self.current_color;
    }
}
