#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl Color {
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn set_red(&mut self, r: f64) {
        self.r = r;
    }

    pub fn red(&self) -> f64 {
        self.r
    }

    pub fn set_green(&mut self, g: f64) {
        self.g = g;
    }

    pub fn green(&self) -> f64 {
        self.g
    }

    pub fn set_blue(&mut self, b: f64) {
        self.b = b;
    }

    pub fn blue(&self) -> f64 {
        self.b
    }

    pub fn set_alpha(&mut self, a: f64) {
        self.a = a;
    }

    pub fn alpha(&self) -> f64 {
        self.a
    }

    pub fn as_tuple(&self) -> (f64, f64, f64, f64) {
        (self.r, self.g, self.b, self.a)
    }
}
