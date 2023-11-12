use crate::color::Color;

pub trait Animation {
    fn is_static(&self) -> bool;
    fn current_frame(&self) -> &Color;
    fn next_frame(&mut self) -> &Color;
}

pub struct Rainbow {
    current_color: Color,
    rotation: u8,
    pub is_static: bool,
}

fn wheel(mut wheel_pos: u8) -> Color {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3, 0).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3, 0).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0, 0).into()
}

impl Rainbow {
    pub const fn new() -> Self {
        Self {
            current_color: Color::default(),
            is_static: false,
            rotation: 0,
        }
    }
}

impl Animation for Rainbow {
    fn next_frame(&mut self) -> &Color {
        self.rotation = self.rotation.wrapping_add(1);

        self.current_color = wheel(self.rotation);
        &self.current_color
    }

    fn current_frame(&self) -> &Color {
        &self.current_color
    }

    fn is_static(&self) -> bool {
        self.is_static
    }
}

pub struct StaticColor {
    pub current_color: Color,
    pub is_static: bool,
}

impl StaticColor {
    pub const fn new(color: Color) -> Self {
        Self {
            current_color: color,
            is_static: true,
        }
    }

    pub const fn default() -> Self {
        StaticColor::new(Color::default())
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }
}

impl Animation for StaticColor {
    fn next_frame(&mut self) -> &Color {
        return self.current_frame();
    }

    fn current_frame(&self) -> &Color {
        &self.current_color
    }

    fn is_static(&self) -> bool {
        self.is_static
    }
}

pub struct AnimationSet {
    pub rainbow: Rainbow,
    pub folowing_static_color: StaticColor,
    pub white_color: StaticColor,
    pub current_index: usize,
}

impl AnimationSet {
    pub fn next_animation(&mut self) {
        self.current_index += 1;
        if self.current_index >= 3 {
            self.current_index = 0;
        }

        if self.current_index == 1 {
            self.folowing_static_color
                .set_color(*self.rainbow.current_frame());
        }
    }

    pub fn current_animation(&mut self) -> &mut dyn Animation {
        match self.current_index {
            0 => &mut self.rainbow,
            1 => &mut self.folowing_static_color,
            2 => &mut self.white_color,
            _ => panic!("Invalid animation index"),
        }
    }

    pub const fn new() -> Self {
        Self {
            rainbow: Rainbow::new(),
            folowing_static_color: StaticColor::default(),
            white_color: StaticColor::new(Color::white()),
            current_index: 0,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.folowing_static_color.set_color(color);
        self.current_index = 1;
    }
}
