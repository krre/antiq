use super::Window;
use super::Windowed;
use crate::core::node::Updatable;

#[derive(Default)]
pub struct ApplicationWindow {
    window: Window,
}

impl Updatable for ApplicationWindow {
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
