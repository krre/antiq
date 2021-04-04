use super::Windowed;
use crate::core::application;

pub struct Window {
    title: String,
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Windowed for Window {
    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn title(&self) -> &str {
        &self.title
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: application::name().unwrap_or("Untitled".to_string()),
        }
    }
}
