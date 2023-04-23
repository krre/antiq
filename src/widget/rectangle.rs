use super::Widget;

pub struct Rectangle {}

impl Rectangle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for Rectangle {
    fn draw(&self) {}
}
