use std::{cell::RefCell, rc::Rc};

use crate::{platform, renderer::Renderer, widget::Widget};

use super::{Color, Context, Pos2D, Size2D};

pub struct Window {
    title: String,
    // surface: gfx::Surface<'static>,
    color: Color,
    position: Pos2D,
    widgets: Vec<RefCell<Box<dyn Widget>>>,
    context: Rc<Context>,
    platform_window: Box<dyn platform::PlatformWindow>,
}

#[derive(Debug)]
pub struct WindowSettings {
    pub title: String,
    pub position: Option<Pos2D>,
    pub size: Option<Size2D>,
    pub color: Color,
    pub maximized: bool,
    pub visible: bool,
}

impl Window {
    pub fn new(
        ctx: Rc<Context>,
        settings: WindowSettings,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let color = settings.color.clone();
        let title = settings.title.clone();
        let position = settings.position.unwrap_or(Pos2D::new(200, 200));

        Ok(Self {
            title,
            color,
            position,
            widgets: Vec::new(),
            context: ctx,
            platform_window: platform::Window::new()?,
        })
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn set_visible(&self, visible: bool) {}

    pub fn set_size(&self, size: Size2D) {}

    pub fn position(&self) -> Pos2D {
        self.position
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_maximized(&self, maximized: bool) {}

    pub fn resize(&mut self, device: &wgpu::Device, size: Size2D) {}

    pub fn build(&self) {
        for widget in &self.widgets {
            widget.borrow().build();
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(RefCell::new(widget));
    }

    pub fn render(&self, renderer: &Renderer) {
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
    fn drop(&mut self) {}
}

impl WindowSettings {
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_size(&mut self, size: Size2D) {
        self.size = Some(size);
    }

    pub fn set_position(&mut self, position: Pos2D) {
        self.position = Some(position);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }
}

impl WindowSettings {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            position: None,
            size: None,
            color: Color::new(0.05, 0.027, 0.15, 1.0),
            maximized: false,
            visible: true,
        }
    }
}
