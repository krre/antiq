use super::Widget;
use crate::core::WidgetCore;
use crate::platform::{self, PlatformWindow};

#[derive(Default)]
pub struct Window {
    widget_core: WidgetCore,
    platform_window: platform::Window,
}

impl Window {
    pub fn new() -> Self {
        Self {
            widget_core: WidgetCore::default(),
            platform_window: platform::Window::default(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.platform_window.set_title(title);
    }
}

impl Widget for Window {
    fn widget_ref(&self) -> &WidgetCore {
        &self.widget_core
    }

    fn widget_mut(&mut self) -> &mut WidgetCore {
        &mut self.widget_core
    }

    fn draw(&self) {}
}
