use super::{Widget, WindowWidget};
use crate::core::WidgetCore;

#[derive(Default, Debug)]
pub struct Window {
    widget_core: WidgetCore,
}

impl Window {
    pub fn new() -> Self {
        Self {
            widget_core: WidgetCore::default(),
        }
    }

    pub fn set_title(&mut self, title: &str) {}
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

impl WindowWidget for Window {}
