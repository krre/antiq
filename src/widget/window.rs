use super::Widget;
use crate::core::EntityCore;
use crate::entity::Entity;
use crate::platform::{self, PlatformWindow};

#[derive(Default)]
pub struct Window {
    entity_core: EntityCore,
    platform_window: platform::Window,
}

impl Window {
    pub fn new() -> Self {
        Self {
            entity_core: EntityCore::default(),
            platform_window: platform::Window::default(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.platform_window.set_title(title);
    }
}

impl Entity for Window {
    fn entity_ref(&self) -> &EntityCore {
        &self.entity_core
    }

    fn entity_mut(&mut self) -> &EntityCore {
        &self.entity_core
    }
}

impl Widget for Window {
    fn draw(&self) {}
}
