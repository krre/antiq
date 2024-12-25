use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use crate::{
    platform,
    renderer::{Renderer, Surface},
    widget::Widget,
};

use super::{Color, Context, Pos2D, Size2D};

pub struct Window {
    title: RefCell<String>,
    color: Cell<Color>,
    position: Cell<Pos2D>,
    size: Cell<Size2D>,
    widgets: Vec<RefCell<Box<dyn Widget>>>,
    context: Rc<Context>,
    platform_window: Box<dyn platform::PlatformWindow>,
    renderer: Rc<Renderer>,
    surface: Surface,
    visible: Cell<bool>,
}

impl Window {
    pub fn new(ctx: Rc<Context>) -> Result<Self, Box<dyn std::error::Error>> {
        let platform_window = platform::Window::new(ctx.platform_context.clone())?;
        let renderer = ctx.renderer().clone();
        let surface = Surface::new(platform_window.as_ref(), &renderer);

        let window = Self {
            title: RefCell::new(String::new()),
            color: Cell::new(Color::new(0.05, 0.027, 0.15, 1.0)),
            position: Cell::new(Pos2D::new(0, 0)),
            size: Cell::new(Size2D::new(0, 0)),
            widgets: Vec::new(),
            context: ctx,
            platform_window,
            renderer,
            surface,
            visible: Cell::new(false),
        };

        window.set_title("Untitled");
        window.set_size(Size2D::new(800, 600));

        Ok(window)
    }

    pub fn set_title(&self, title: &str) {
        self.platform_window.set_title(title);
        *self.title.borrow_mut() = title.to_string();
    }

    pub fn title(&self) -> String {
        self.title.borrow().clone()
    }

    pub fn set_visible(&self, visible: bool) {
        self.platform_window.set_visible(visible);
        self.visible.set(true);
    }

    pub fn is_visible(&self) -> bool {
        self.visible.get()
    }

    pub fn set_position(&self, pos: Pos2D) {
        self.platform_window.set_position(pos);
        self.position.set(pos);
    }

    pub fn position(&self) -> Pos2D {
        self.position.get()
    }

    pub fn set_size(&self, size: Size2D) {
        self.platform_window.set_size(size);
        self.size.set(size);
    }

    pub fn size(&self) -> Size2D {
        self.size.get()
    }

    pub fn set_color(&self, color: Color) {
        self.color.set(color);
    }

    pub fn color(&self) -> Color {
        self.color.get()
    }

    pub fn set_maximized(&self, maximized: bool) {}

    pub fn build(&self) {
        for widget in &self.widgets {
            widget.borrow().build();
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(RefCell::new(widget));
    }

    pub fn render(&self) {
        println!("Render window: {}", self.title());

        let frame = self.surface.current_frame();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.renderer.clear_view(&view, &self.color.get().inner());
        frame.present();
    }
}

impl Drop for Window {
    fn drop(&mut self) {}
}
