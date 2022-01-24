use super::{Widget, WindowWidget};
use crate::core::WidgetCore;
use crate::entity::Application;

use winit;

#[derive(Debug)]
pub struct Window {
    widget_core: WidgetCore,
    window: winit::window::Window,
}

impl Window {
    pub fn new(app: &Application) -> Result<Self, Box<dyn std::error::Error>> {
        let window = winit::window::Window::new(app.event_loop())?;

        Ok(Self {
            widget_core: WidgetCore::default(),
            window,
        })
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
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

impl WindowWidget for Window {}
