use super::Windowed;

pub struct Window {
    title: String,
}

impl Window {
    pub fn new() -> Self {
        Window::default()
    }
}

impl Windowed for Window {
    fn set_title(&mut self, title: &str) {
        self.title = title.into();
    }

    fn title(&self) -> &str {
        &self.title
    }
}

impl Default for Window {
    fn default() -> Self {
        Window {
            title: "Untitled".to_string(),
        }
    }
}
