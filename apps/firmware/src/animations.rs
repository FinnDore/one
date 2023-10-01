use smart_leds::RGB8;

pub trait NextFrame {
    fn next_frame(&mut self) -> &RGB8;
}

pub trait CurrentFrame {
    fn current_frame(&self) -> &RGB8;
}

struct RainbowAnimation {
    current_color: RGB8,
    pub is_static: bool,
}

pub struct StaticColorAnimation {
    pub colors: [RGB8; 10],
    pub current_index: usize,
    pub is_static: bool,
}

impl StaticColorAnimation {
    pub fn new(colors: [RGB8; 10]) -> Self {
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

impl CurrentFrame for StaticColorAnimation {
    fn current_frame(&self) -> &RGB8 {
        return self.colors.get(self.current_index).unwrap();
    }
}

impl NextFrame for RainbowAnimation {
    fn next_frame(&mut self) -> &RGB8 {
        return &self.current_color;
    }
}

impl CurrentFrame for RainbowAnimation {
    fn current_frame(&self) -> &RGB8 {
        return &self.current_color;
    }
}
