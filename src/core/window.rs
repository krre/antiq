use std::cell::RefCell;

use crate::{
    gfx::{self, Gpu},
    widget::Widget,
};
use winit;

use super::{Application, Color, Position, Size};

#[derive(Debug, Clone, Copy)]
pub struct Id(winit::window::WindowId);

pub type DropHandler = dyn Fn(&Window);

pub struct Window {
    id: Id,
    title: String,
    winit_window: winit::window::Window,
    // surface: gfx::Surface<'static>,
    color: Color,
    position: Position,
    widgets: Vec<RefCell<Box<dyn Widget>>>,
    drop_hanlder: Option<Box<DropHandler>>,
}

pub struct Settings {
    title: String,
    position: Option<Position>,
    size: Option<Size>,
    color: Color,
    maximized: bool,
    visible: bool,
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
    pub(crate) fn new(app: &Application, settings: Settings) -> Self {
        let mut window_attributes = winit::window::Window::default_attributes()
            .with_title(&settings.title)
            .with_visible(settings.visible)
            .with_maximized(settings.maximized);

        let position = if let Some(position) = settings.position {
            window_attributes = window_attributes.with_position(winit::dpi::PhysicalPosition::new(
                position.x(),
                position.y(),
            ));
            position
        } else {
            Position::new(200, 200)
        };

        if let Some(size) = settings.size {
            window_attributes = window_attributes
                .with_inner_size(winit::dpi::PhysicalSize::new(size.width, size.height));
        }

        let winit_window = app.event_loop().create_window(window_attributes).unwrap();

        let id = Id::new(winit_window.id());
        // let surface = app.engine().gpu().create_surface(&winit_window);

        Self {
            id,
            title: settings.title.clone(),
            winit_window,
            // surface,
            color: settings.color,
            position,
            widgets: Vec::new(),
            drop_hanlder: None,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn set_title(&mut self, title: &str) {
        self.winit_window.set_title(title);
        self.title = String::from(title);
    }

    pub fn set_visible(&self, visible: bool) {
        self.winit_window.set_visible(visible);
    }

    pub fn set_size(&self, size: Size) {
        self.winit_window
            .request_inner_size(winit::dpi::PhysicalSize::new(size.width, size.height));
    }

    pub fn size(&self) -> Size {
        Size::new(
            self.winit_window.inner_size().width,
            self.winit_window.inner_size().height,
        )
    }

    pub fn set_position(&mut self, position: Position) {
        self.winit_window
            .set_outer_position(winit::dpi::PhysicalPosition::new(
                position.x(),
                position.y(),
            ));
        self.set_cache_position(position)
    }

    pub(crate) fn set_cache_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_maximized(&self, maximized: bool) {
        self.winit_window.set_maximized(maximized);
    }

    pub fn is_maximized(&self) -> bool {
        self.winit_window.is_maximized()
    }

    pub fn set_drop_handler(&mut self, handler: Box<DropHandler>) {
        self.drop_hanlder = Some(handler);
    }

    pub fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        // self.surface.resize(device, size.width, size.height);
        self.winit_window.request_redraw();
    }

    pub fn build(&self) {
        log::info!("Build window: {}", self.winit_window.title());

        for widget in &self.widgets {
            widget.borrow().build();
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(RefCell::new(widget));
    }

    pub fn render(&self, gpu: &Gpu) {
        log::info!("Render window: {}", self.title);

        // let frame = self.surface.current_frame();
        // let view = frame
        //     .texture
        //     .create_view(&wgpu::TextureViewDescriptor::default());
        // gpu.clear_view(&view, &self.color.inner());
        // frame.present();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if let Some(dh) = &self.drop_hanlder {
            dh(self);
        }
    }
}

impl Settings {
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = Some(size);
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            position: None,
            size: None,
            color: Color::new(0.5, 0.5, 0.5, 1.0),
            maximized: false,
            visible: true,
        }
    }
}
