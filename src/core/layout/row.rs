use super::Layout;

pub struct Row {}

impl Row {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Row {
    fn draw(&self) {}
}
