use super::{Widget, Window, WindowWidget};
use crate::core::WidgetCore;
use crate::entity::Application;

#[derive(Debug)]
pub struct ApplicationWindow {
    window: Window,
}

impl ApplicationWindow {
    pub fn new(app: &mut Application) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Window::new(app)?;
        Ok(Self { window })
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.window.set_visible(visible);
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

impl WindowWidget for ApplicationWindow {
    fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    fn render(&self) {
        self.window.render();
        println!("Application window render");
    }
}
