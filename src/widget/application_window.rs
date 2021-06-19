use super::Widget;
use super::Window;
use crate::core::Id;
use crate::entity::Entity;

pub struct ApplicationWindow {
    window: Window,
}

impl ApplicationWindow {
    pub fn new() -> Self {
        Self {
            window: Window::default(),
        }
    }
}

impl Default for ApplicationWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for ApplicationWindow {
    fn id(&self) -> Id {
        self.window.id()
    }
}

impl Widget for ApplicationWindow {
    fn draw(&self) {
        self.window.draw();
    }
}
