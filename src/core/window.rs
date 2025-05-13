use std::{
    cell::RefCell,
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
    title: String,
    color: Color,
    position: Pos2D,
    size: Size2D,
    platform_window: Box<dyn platform::PlatformWindow>,
    window_manager: Weak<RefCell<WindowManager>>,
    renderer: Weak<Renderer>,
    surface: Surface,
    visible: bool,
    maximized: bool,
    layout: Box<dyn Layout>,
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
    pub fn new(application: &Application) -> Result<Weak<RefCell<Self>>> {
        let size = Size2D::new(800, 600);
        let platform_window = platform::Window::new(
            application.platform_application.clone(),
            application.event_loop().platform_event_loop.clone(),
            size.clone(),
        )?;
        let renderer = application.renderer().clone();
        let surface = Surface::new(platform_window.as_ref(), &renderer.upgrade().unwrap());

        let window = Rc::new(RefCell::new(Self {
            title: String::new(),
            color: Color::new(0.05, 0.027, 0.15),
            position: Pos2D::default(),
            size: Size2D::default(),
            platform_window,
            window_manager: application.window_manager().clone(),
            renderer,
            surface,
            visible: false,
            maximized: false,
            layout: Box::new(Fill::new()),
        }));

        application
            .window_manager()
            .upgrade()
            .map(|wm| wm.borrow_mut().append(window.borrow().id(), window.clone()));

        {
            let mut w = window.borrow_mut();
            w.set_visible(true);
            w.set_title("Untitled");
            w.set_size(size);
        }

        Ok(Rc::downgrade(&window))
    }

    pub fn id(&self) -> WindowId {
        self.platform_window.id()
    }

    pub fn set_title(&mut self, title: &str) {
        self.platform_window.set_title(title);
        self.title = title.to_string();
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.platform_window.set_visible(visible);
        self.visible = true;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_position(&mut self, pos: Pos2D) {
        let border = self.border();
        let correct_pos = Pos2D::new(pos.x - border.left as i32, pos.y - border.top as i32);

        self.platform_window.set_position(correct_pos);
        self.position = correct_pos;
    }

    pub(crate) fn update_position(&mut self, pos: Pos2D) {
        self.position = pos;
    }

    pub fn position(&self) -> Pos2D {
        self.position
    }

    pub fn set_size(&mut self, size: Size2D) {
        self.platform_window.set_size(size);
        self.surface
            .set_size(self.renderer.upgrade().unwrap().device(), size);
        self.size = size;
    }

    pub(crate) fn update_size(&mut self, size: Size2D) {
        self.surface
            .set_size(self.renderer.upgrade().unwrap().device(), size);
        self.size = size;
    }

    pub fn size(&self) -> Size2D {
        self.size
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.render();
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn border(&self) -> Border2D {
        self.platform_window.border()
    }

    pub fn set_maximized(&mut self, maximized: bool) {
        self.maximized = maximized;
    }

    pub fn maximized(&self) -> bool {
        self.maximized
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }

    pub fn layout(&self) -> &Box<dyn Layout> {
        &self.layout
    }

    pub fn render(&self) {
        let frame = self.surface.current_frame();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.renderer
            .upgrade()
            .unwrap()
            .clear_view(&view, self.color);
        frame.present();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window_manager
            .upgrade()
            .map(|wm| wm.try_borrow_mut().map(|mut wm| wm.remove(self.id())));
    }
}
