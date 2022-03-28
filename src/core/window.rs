use crate::core::Application;
use std::rc::Rc;
use winit;

#[derive(Debug)]
pub struct Window {
    window: winit::window::Window,
}

impl Window {
    pub fn new() -> Rc<Self> {
        let winit_window = winit::window::WindowBuilder::new()
            .with_visible(false)
            .build(&Application::event_loop())
            .unwrap();

        Rc::new(Self {
            window: winit_window,
        })
    }

    pub fn set_title(self: &Rc<Self>, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_visible(self: &Rc<Self>, visible: bool) {
        self.window.set_visible(visible);

        if visible {
            Application::add_window(Rc::downgrade(self));
        }
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
