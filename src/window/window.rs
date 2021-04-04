use super::Windowed;

pub struct Window {
    title: String,
}

impl Window {
    pub fn new() -> Self {
        Self::default()
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
        Self {
            title: application_name().unwrap_or("Untitled".to_string()),
        }
    }
}

fn application_name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}
