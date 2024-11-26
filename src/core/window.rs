use std::{cell::RefCell, rc::Rc};

use crate::{renderer::Renderer, widget::Widget};
use winit;

use super::{application::UserEvent, AppContext, Color, Pos2d, Size2d};

#[derive(Debug, Clone, Copy)]
pub struct Id(winit::window::WindowId);

pub struct Window {
    id: Id,
    title: String,
    // surface: gfx::Surface<'static>,
    color: Color,
    position: Pos2d,
    widgets: Vec<RefCell<Box<dyn Widget>>>,
    context: Rc<AppContext>,
}

#[derive(Debug)]
pub struct WindowSettings {
    pub title: String,
    pub position: Option<Pos2d>,
    pub size: Option<Size2d>,
    pub color: Color,
    pub maximized: bool,
    pub visible: bool,
}

impl Id {
    pub(crate) fn new(winit_id: winit::window::WindowId) -> Self {
        Self(winit_id)
    }

    pub(crate) fn inner(&self) -> winit::window::WindowId {
        self.0
    }
}

impl Window {
    pub fn new(ctx: Rc<AppContext>, settings: WindowSettings) -> Self {
        // let winit_window = ctx.event_loop_proxy(). app.event_loop().create_window(window_attributes).unwrap();

        // let id = Id::new(winit_window.id());
        // let surface = app.engine().gpu().create_surface(&winit_window);

        let color = settings.color.clone();
        let title = settings.title.clone();
        let position = settings.position.unwrap_or(Pos2d::new(200, 200));

        ctx.event_loop_proxy()
            .send_event(UserEvent::CreateWindow(settings))
            .expect("Event loop closed");

        Self {
            id: Id::new(winit::window::WindowId::dummy()),
            title,
            color,
            position,
            widgets: Vec::new(),
            context: ctx,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn set_title(&mut self, title: &str) {
        // self.winit_window.set_title(title);
        self.title = String::from(title);
    }

    pub fn set_visible(&self, visible: bool) {
        // self.winit_window.set_visible(visible);
    }

    pub fn set_size(&self, size: Size2d) {
        // self.winit_window.request_inner_size(winit::dpi::PhysicalSize::new(size.width, size.height));
    }

    // pub fn size(&self) -> Size {
    //     Size::new(
    //         self.winit_window.inner_size().width,
    //         self.winit_window.inner_size().height,
    //     )
    // }

    // pub fn set_position(&mut self, position: Position) {
    //     self.winit_window
    //         .set_outer_position(winit::dpi::PhysicalPosition::new(
    //             position.x(),
    //             position.y(),
    //         ));
    // }

    pub fn position(&self) -> Pos2d {
        self.position
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_maximized(&self, maximized: bool) {
        // self.winit_window.set_maximized(maximized);
    }

    // pub fn is_maximized(&self) -> bool {
    // self.winit_window.is_maximized()
    // }

    pub fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        // self.surface.resize(device, size.width, size.height);
        // self.winit_window.request_redraw();
    }

    pub fn build(&self) {
        // log::info!("Build window: {}", self.winit_window.title());

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

    pub fn set_size(&mut self, size: Size2d) {
        self.size = Some(size);
    }

    pub fn set_position(&mut self, position: Pos2d) {
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
