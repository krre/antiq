use super::Widget;
use super::Window;
use crate::core::EntityCore;
use crate::entity::Entity;

#[derive(Default)]
pub struct ApplicationWindow {
    window: Window,
}

impl ApplicationWindow {
    pub fn new() -> Self {
        Self {
            window: Window::default(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
}

impl Entity for ApplicationWindow {
    fn entity_ref(&self) -> &EntityCore {
        self.window.entity_ref()
    }

    fn entity_mut(&mut self) -> &EntityCore {
        self.window.entity_mut()
    }
}

impl Widget for ApplicationWindow {
    fn draw(&self) {
        self.window.draw();
    }
}
