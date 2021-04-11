use super::Windowed;
use crate::core::application;
use crate::platform::window as platform_window;

pub struct Window {
    title: String,
    platform_window: platform_window::Window,
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Windowed for Window {
    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.platform_window.set_title(title);
    }

    fn title(&self) -> &str {
        &self.title
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: application::name().unwrap_or("Untitled".to_string()),
            platform_window: platform_window::Window::new(),
        }
    }
}
