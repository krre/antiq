use crate::core::Application;
use winit;

#[derive(Debug, Clone, Copy)]
pub struct Id(winit::window::WindowId);

#[derive(Debug)]
pub struct Window {
    window: winit::window::Window,
    id: Id,
}

impl Id {
    pub(crate) fn new(winit_id: winit::window::WindowId) -> Self {
        Self(winit_id)
    }

    pub(crate) fn winit_id(&self) -> winit::window::WindowId {
        self.0
    }
}

impl Window {
    pub(crate) fn new(app: &Application) -> Self {
        let winit_window = winit::window::WindowBuilder::new()
            .build(&app.event_loop())
            .unwrap();

        let id = Id::new(winit_window.id());

        Self {
            window: winit_window,
            id,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_visible(&self, visible: bool) {
        self.window.set_visible(visible);
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.window
            .set_inner_size(winit::dpi::PhysicalSize::new(width, height));
    }

    pub fn set_position(&self, x: i32, y: i32) {
        self.window
            .set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
    }

    pub fn draw(&self) {
        println!("Window draw");
    }
}