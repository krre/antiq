use super::Widget;
use crate::core::Id;
use crate::entity::Entity;
use crate::platform::{self, PlatformWindow};

#[derive(Default)]
pub struct Window {
    id: Id,
    platform_window: platform::Window,
}

impl Window {
    pub fn new() -> Self {
        Self {
            id: Id::default(),
            platform_window: platform::Window::default(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.platform_window.set_title(title);
    }
}

impl Entity for Window {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Window {
    fn draw(&self) {}
}
