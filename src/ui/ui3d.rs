use crate::{event::EventHandler, ui::Color};

pub struct Ui3d {
    backgroud_color: Color,
}

impl Ui3d {
    pub fn new() -> Self {
        Self {
            backgroud_color: Color::new(0.0, 0.0, 1.1),
        }
    }

    pub fn set_backgroud_color(&mut self, color: &Color) {
        self.backgroud_color = color.clone();
    }

    pub fn backgroud_color(&self) -> &Color {
        &self.backgroud_color
    }
}

impl EventHandler for Ui3d {}
