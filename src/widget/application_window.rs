use super::Widget;
use super::Window;
use crate::core::WidgetCore;

#[derive(Default)]
pub struct ApplicationWindow {
    window: Window,
}

impl ApplicationWindow {
    pub fn new() -> Self {
        Self {
            window: Window::default(),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
}

impl Widget for ApplicationWindow {
    fn widget_ref(&self) -> &WidgetCore {
        self.window.widget_ref()
    }

    fn widget_mut(&mut self) -> &mut WidgetCore {
        self.window.widget_mut()
    }

    fn draw(&self) {
        self.window.draw();
    }
}
