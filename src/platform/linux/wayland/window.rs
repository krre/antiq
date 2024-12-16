use std::any::Any;

use crate::platform::PlatformWindow;

pub struct Window {}

impl Window {
    pub fn new() -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_title(&self, title: &str) {}

    fn title(&self) -> String {
        String::new()
    }

    fn set_visible(&self, visible: bool) {}
}
