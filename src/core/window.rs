use crate::core::Application;
use std::cell::RefCell;
use std::rc::Weak;
use winit;

#[derive(Debug)]
pub struct Window {
    window: winit::window::Window,
}

impl Window {
    pub fn new() -> Result<Weak<RefCell<Self>>, Box<dyn std::error::Error>> {
        let window = winit::window::WindowBuilder::new()
            .with_visible(false)
            .build(&Application::event_loop())?;

        let win = Self { window };
        let weak_win = Application::add_window(win);

        Ok(weak_win)
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.window.set_visible(visible);
    }
}

impl Window {
    pub fn id(&self) -> winit::window::WindowId {
        self.window.id()
    }

    pub fn render(&self) {
        println!("Window render");
    }
}
