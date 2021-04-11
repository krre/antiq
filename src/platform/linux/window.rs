use crate::window::Windowed;

pub struct Window {}

impl Window {
    pub fn new() -> Self {
        Self {}
    }
}

impl Windowed for Window {
    fn set_title(&mut self, title: &str) {}

    fn title(&self) -> &str {
        "Linux window"
    }
}
