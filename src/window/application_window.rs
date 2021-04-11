use super::Window;
use super::Windowed;

pub struct ApplicationWindow {
    window: Window,
}

impl ApplicationWindow {
    pub fn new() -> Self {
        Self {
            window: Window::new(),
        }
    }
}

impl Windowed for ApplicationWindow {
    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    fn title(&self) -> &str {
        self.window.title()
    }
}
