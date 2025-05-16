use crate::core::Color;

use super::Widget;

pub struct Rectangle {
    color: Color,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Widget for Rectangle {
    fn build(&self) {}
}
