use std::{
    cell::{Cell, RefCell},
    rc::{Rc, Weak},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    platform,
    renderer::{Renderer, Surface},
    ui::layout::{Layout, fill::Fill},
};

use super::{
    Border2D, Color, Pos2D, Result, Size2D, application::Application, window_manager::WindowManager,
};

static ID_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Hash, Debug)]
pub struct WindowId(usize);

pub struct Window {
    title: RefCell<String>,
    color: Cell<Color>,
    position: Cell<Pos2D>,
    size: Cell<Size2D>,
    platform_window: Box<dyn platform::PlatformWindow>,
    window_manager: Rc<WindowManager>,
    renderer: Rc<Renderer>,
    surface: RefCell<Surface>,
    visible: Cell<bool>,
    maximized: Cell<bool>,
    layout: RefCell<Box<dyn Layout>>,
}

impl WindowId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn generate_new() -> Self {
        Self(ID_COUNT.fetch_add(1, Ordering::Relaxed))
    }

    pub fn inner(&self) -> usize {
        self.0
    }
}

impl PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for WindowId {}

impl Window {
    pub fn new(application: &Application) -> Result<Weak<Self>> {
        let size = Size2D::new(800, 600);
        let platform_window = platform::Window::new(
            application.platform_application.clone(),
            application.event_loop().platform_event_loop.clone(),
            size.clone(),
        )?;
        let renderer = application.renderer().clone();
        let surface = RefCell::new(Surface::new(platform_window.as_ref(), &renderer));

        let window = Rc::new(Self {
            title: RefCell::new(String::new()),
            color: Cell::new(Color::new(0.05, 0.027, 0.15)),
            position: Cell::new(Pos2D::default()),
            size: Cell::new(Size2D::default()),
            platform_window,
            window_manager: application.window_manager(),
            renderer,
            surface,
            visible: Cell::new(false),
            maximized: Cell::new(false),
            layout: RefCell::new(Box::new(Fill::new())),
        });

        application
            .window_manager()
            .append(window.id(), window.clone());

        window.set_visible(true);
        window.set_title("Untitled");
        window.set_size(size);

        Ok(Rc::downgrade(&window))
    }

    pub fn id(&self) -> WindowId {
        self.platform_window.id()
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
        let border = self.border();
        let correct_pos = Pos2D::new(pos.x - border.left as i32, pos.y - border.top as i32);

        self.platform_window.set_position(correct_pos);
        self.position.set(correct_pos);
    }

    pub(crate) fn update_position(&self, pos: Pos2D) {
        self.position.set(pos);
    }

    pub fn position(&self) -> Pos2D {
        self.position.get()
    }

    pub fn set_size(&self, size: Size2D) {
        self.platform_window.set_size(size);
        self.surface
            .borrow_mut()
            .set_size(self.renderer.device(), size);
        self.size.set(size);
    }

    pub(crate) fn update_size(&self, size: Size2D) {
        self.surface
            .borrow_mut()
            .set_size(self.renderer.device(), size);
        self.size.set(size);
    }

    pub fn size(&self) -> Size2D {
        self.size.get()
    }

    pub fn set_color(&self, color: Color) {
        self.color.set(color);
        self.render();
    }

    pub fn color(&self) -> Color {
        self.color.get()
    }

    pub fn border(&self) -> Border2D {
        self.platform_window.border()
    }

    pub fn set_maximized(&self, maximized: bool) {
        self.maximized.set(maximized);
    }

    pub fn maximized(&self) -> bool {
        self.maximized.get()
    }

    pub fn set_layout(&self, layout: Box<dyn Layout>) {
        *self.layout.borrow_mut() = layout;
    }

    pub fn render(&self) {
        let frame = self.surface.borrow().current_frame();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.renderer.clear_view(&view, self.color.get());
        frame.present();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window_manager.remove(self.id());
    }
}
