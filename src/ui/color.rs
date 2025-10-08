#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };

    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn set_red(&mut self, r: f32) {
        self.r = r;
    }

    pub fn red(&self) -> f32 {
        self.r
    }

    pub fn set_green(&mut self, g: f32) {
        self.g = g;
    }

    pub fn green(&self) -> f32 {
        self.g
    }

    pub fn set_blue(&mut self, b: f32) {
        self.b = b;
    }

    pub fn blue(&self) -> f32 {
        self.b
    }

    pub fn set_alpha(&mut self, a: f32) {
        self.a = a;
    }

    pub fn alpha(&self) -> f32 {
        self.a
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }
}
