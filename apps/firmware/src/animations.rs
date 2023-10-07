use smart_leds::RGB8;

use crate::COLORS;

pub trait NextAndCurrentFrame {
    fn is_static(&self) -> bool;
    fn current_frame(&self) -> &RGB8;
    fn next_frame(&mut self) -> &RGB8;
}

pub struct RainbowAnimation {
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

impl RainbowAnimation {
    pub const fn new() -> Self {
        Self {
            current_color: RGB8::new(255, 255, 255),
            is_static: false,
            rotation: 0,
        }
    }
}

impl NextAndCurrentFrame for RainbowAnimation {
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

pub struct StaticColorAnimation {
    pub colors: [RGB8; 10],
    pub current_index: usize,
    pub is_static: bool,
}

impl StaticColorAnimation {
    pub const fn new(colors: [RGB8; 10]) -> Self {
        Self {
            colors,
            current_index: 0,
            is_static: true,
        }
    }
}

impl NextAndCurrentFrame for StaticColorAnimation {
    fn next_frame(&mut self) -> &RGB8 {
        self.current_index += 1;
        if self.current_index >= self.colors.len() - 1 {
            self.current_index = 0;
        }

        return self.colors.get(self.current_index).unwrap();
    }

    fn current_frame(&self) -> &RGB8 {
        return self.colors.get(self.current_index).unwrap();
    }

    fn is_static(&self) -> bool {
        return self.is_static;
    }
}

pub struct BigStaticColorAnimation {
    pub current_color: RGB8,
    pub is_static: bool,
}

impl BigStaticColorAnimation {
    pub const fn new() -> Self {
        Self {
            current_color: RGB8::new(255, 255, 255),
            is_static: true,
        }
    }

    pub fn set_color(&mut self, color: RGB8) {
        self.current_color = color;
    }
}

impl NextAndCurrentFrame for BigStaticColorAnimation {
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
    pub rainbow: RainbowAnimation,
    pub static_color: StaticColorAnimation,
    pub folowing_static_color: BigStaticColorAnimation,
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
        } else if self.current_index == 2 {
            self.static_color.next_frame();
        }
    }

    pub fn current_animation(&mut self) -> &mut dyn NextAndCurrentFrame {
        match self.current_index {
            0 => &mut self.rainbow,
            1 => &mut self.folowing_static_color,
            _ => panic!("Invalid animation index"),
        }
    }

    pub const fn new() -> Self {
        Self {
            rainbow: RainbowAnimation::new(),
            folowing_static_color: BigStaticColorAnimation::new(),
            static_color: StaticColorAnimation::new(COLORS),
            current_index: 0,
        }
    }
}
