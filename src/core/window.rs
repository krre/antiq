use crate::core::Application;
use std::rc::Rc;
use winit;

#[derive(Debug, Clone, Copy)]
pub struct Id(winit::window::WindowId);

#[derive(Debug)]
pub struct Window {
    window: winit::window::Window,
    id: Id,
}

impl Id {
    pub(crate) fn winit_id(&self) -> winit::window::WindowId {
        self.0
    }
}

impl Window {
    pub fn new() -> Rc<Self> {
        let winit_window = winit::window::WindowBuilder::new()
            .with_visible(false)
            .build(&Application::event_loop())
            .unwrap();

        let id = Id(winit_window.id());

        Rc::new(Self {
            window: winit_window,
            id,
        })
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub(crate) fn winit_id(&self) -> winit::window::WindowId {
        self.id.0
    }

    pub fn set_title(self: &Rc<Self>, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_visible(self: &Rc<Self>, visible: bool) {
        self.window.set_visible(visible);

        if visible {
            Application::add_window(Rc::downgrade(self));
        } else {
            Application::remove_window(&self.id());
        }
    }

    pub fn set_size(self: &Rc<Self>, width: u32, height: u32) {
        self.window
            .set_inner_size(winit::dpi::PhysicalSize::new(width, height));
    }

    pub fn set_position(self: &Rc<Self>, x: i32, y: i32) {
        self.window
            .set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
    }

    pub fn draw(&self) {
        println!("Window draw");
    }
}
