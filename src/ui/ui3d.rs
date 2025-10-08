use crate::{
    event::EventHandler,
    ui::{
        Color,
        d2::layout::{Fill2D, Layout2D},
    },
};

pub struct Ui3d {
    background_color: Color,
    layout: Box<dyn Layout2D>,
}

impl Ui3d {
    pub fn new() -> Self {
        Self {
            background_color: Color::from_rgb(0.0, 0.0, 1.1),
            layout: Box::new(Fill2D::new()),
        }
    }

    pub fn set_background_color(&mut self, color: &Color) {
        self.background_color = color.clone();
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout2D>) {
        self.layout = layout;
    }

    pub fn layout(&self) -> &Box<dyn Layout2D> {
        &self.layout
    }
}

impl EventHandler for Ui3d {}
