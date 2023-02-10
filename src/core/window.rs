use crate::core::Application;
use winit;

#[derive(Debug, Clone, Copy)]
pub struct Id(winit::window::WindowId);

#[derive(Debug)]
pub struct Window {
    id: Id,
    winit_window: winit::window::Window,
    wgpu_surface: wgpu::Surface,
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
        let wgpu_surface = unsafe { app.wgpu_instance().create_surface(&winit_window).unwrap() };

        Self {
            id,
            winit_window,
            wgpu_surface,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn set_title(&self, title: &str) {
        self.winit_window.set_title(title);
    }

    pub fn set_visible(&self, visible: bool) {
        self.winit_window.set_visible(visible);
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.winit_window
            .set_inner_size(winit::dpi::PhysicalSize::new(width, height));
    }

    pub fn set_position(&self, x: i32, y: i32) {
        self.winit_window
            .set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
    }

    pub fn draw(&self) {
        println!("Window draw");
    }
}
