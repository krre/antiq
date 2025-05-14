use super::Widget;

pub struct View {}

impl View {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for View {
    fn build(&self) {}
}
