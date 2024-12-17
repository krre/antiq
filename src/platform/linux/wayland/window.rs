use std::any::Any;

use crate::{
    core::{Pos2D, Size2D},
    platform::PlatformWindow,
};

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

    fn set_position(&self, pos: Pos2D) {}

    fn set_size(&self, size: Size2D) {}
}
