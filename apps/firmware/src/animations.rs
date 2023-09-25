use smart_leds::{colors::AQUA, RGB8};

pub trait Animation {
    fn is_static(&self) -> bool;
    fn current_frame(&self) -> &RGB8;
    fn next_frame(&mut self) -> &RGB8;
}

pub struct Rainbow {
    current_color: RGB8,
    rotation: u8,
    pub is_static: bool,
}

fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}

impl Rainbow {
    pub const fn new() -> Self {
        Self {
            current_color: RGB8::new(255, 255, 255),
            is_static: false,
            rotation: 0,
        }
    }
}

impl Animation for Rainbow {
    fn next_frame(&mut self) -> &RGB8 {
        if self.rotation >= 255 {
            self.rotation = 0;
        } else {
            self.rotation += 1;
        }

        self.current_color = wheel(self.rotation);
        return &self.current_color;
    }

    fn current_frame(&self) -> &RGB8 {
        return &self.current_color;
    }

    fn is_static(&self) -> bool {
        return self.is_static;
    }
}

pub struct StaticColor {
    pub current_color: RGB8,
    pub is_static: bool,
}

impl StaticColor {
    pub const fn new(color: RGB8) -> Self {
        Self {
            current_color: color,
            is_static: true,
        }
    }

    pub const fn default() -> Self {
        StaticColor::new(RGB8::new(255, 255, 255))
    }

    pub fn set_color(&mut self, color: RGB8) {
        self.current_color = color;
    }
}

impl Animation for StaticColor {
    fn next_frame(&mut self) -> &RGB8 {
        return self.current_frame();
    }

    fn current_frame(&self) -> &RGB8 {
        return &self.current_color;
    }

    fn is_static(&self) -> bool {
        return self.is_static;
    }
}

pub struct AnimationSet {
    pub rainbow: Rainbow,
    pub static_color: StaticColor,
    pub folowing_static_color: StaticColor,
    pub current_index: usize,
}

impl AnimationSet {
    pub fn next_animation(&mut self) {
        self.current_index += 1;
        if self.current_index >= 2 {
            self.current_index = 0;
        }

        if self.current_index == 1 {
            self.folowing_static_color
                .set_color(self.rainbow.current_frame().clone());
        }
    }

    pub fn current_animation(&mut self) -> &mut dyn Animation {
        match self.current_index {
            0 => &mut self.rainbow,
            1 => &mut self.folowing_static_color,
            _ => panic!("Invalid animation index"),
        }
    }

    pub const fn new() -> Self {
        Self {
            rainbow: Rainbow::new(),
            folowing_static_color: StaticColor::default(),
            static_color: StaticColor::new(AQUA),
            current_index: 0,
        }
    }
}
