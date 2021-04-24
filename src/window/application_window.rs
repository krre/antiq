use super::Window;
use super::Windowed;
use crate::core::node::Update;

#[derive(Default)]
pub struct ApplicationWindow {
    window: Window,
}

impl Update for ApplicationWindow {
    fn update(&mut self) {}
}

impl Windowed for ApplicationWindow {
    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    fn title(&self) -> &str {
        self.window.title()
    }
}
