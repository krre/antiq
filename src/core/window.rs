use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    platform,
    renderer::{Renderer, Surface},
    ui::{
        Color,
        d2::{
            geometry::{Border2D, Pos2D, Size2D},
            layout::{Fill2D, Layout2D},
        },
    },
};

use super::{Result, application::Application, window_manager::WindowManager};

static ID_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct WindowId(usize);

pub struct Window {
    title: String,
    geometry: Rc<RefCell<WindowGeometry>>,
    platform_window: Rc<dyn platform::PlatformWindow>,
    window_manager: Weak<RefCell<WindowManager>>,
    visible: bool,
    maximized: bool,
    layout: Box<dyn Layout2D>,
}

pub(crate) struct WindowGeometry {
    position: Pos2D,
    size: Size2D,
    color: Color,
    surface: Surface,
    renderer: Weak<Renderer>,
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

impl WindowGeometry {
    pub fn set_size(&mut self, size: Size2D) {
        self.surface
            .set_size(self.renderer.upgrade().unwrap().device(), size);
        self.size = size;
    }

    pub(crate) fn update_size(&mut self, size: Size2D) {
        self.surface
            .set_size(self.renderer.upgrade().unwrap().device(), size);
        self.size = size;
    }

    pub(crate) fn update_position(&mut self, pos: Pos2D) {
        self.position = pos;
    }

    pub fn render(&self) {
        let frame = self.surface.current_frame();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        if let Some(r) = self.renderer.upgrade() {
            r.clear_view(&view, self.color)
        }

        frame.present();
    }
}

impl Window {
    pub fn new(application: &Application) -> Result<Self> {
        let size = Size2D::new(800, 600);
        let platform_window = platform::new_window(
            application.platform_application.clone(),
            application.event_loop().platform_event_loop.clone(),
            size,
        )?;
        let renderer = application.renderer().clone();
        let surface = Surface::new(platform_window.as_ref(), &renderer.upgrade().unwrap());

        let geometry = Rc::new(RefCell::new(WindowGeometry {
            position: Pos2D::default(),
            size: Size2D::default(),
            color: Color::new(0.05, 0.027, 0.15),
            surface,
            renderer,
        }));

        let mut window = Self {
            title: String::new(),
            geometry: geometry.clone(),
            platform_window,
            window_manager: application.window_manager().clone(),
            visible: false,
            maximized: false,
            layout: Box::new(Fill2D::new()),
        };

        if let Some(wm) = application.window_manager().upgrade() {
            wm.borrow_mut().append(window.id(), geometry)
        }

        window.set_visible(true);
        window.set_title("Untitled");
        window.set_size(size);

        Ok(window)
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
        let correct_pos = Pos2D::new(
            pos.x() - border.left() as i32,
            pos.y() - border.top() as i32,
        );

        self.platform_window.set_position(correct_pos);
        self.geometry.borrow_mut().position = correct_pos;
    }

    pub fn position(&self) -> Pos2D {
        self.geometry.borrow().position
    }

    pub fn set_size(&mut self, size: Size2D) {
        self.platform_window.set_size(size);
        self.geometry.borrow_mut().set_size(size);
    }

    pub fn size(&self) -> Size2D {
        self.geometry.borrow().size
    }

    pub fn set_color(&mut self, color: Color) {
        self.geometry.borrow_mut().color = color;
        self.render();
    }

    pub fn color(&self) -> Color {
        self.geometry.borrow().color
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

    pub fn set_layout(&mut self, layout: Box<dyn Layout2D>) {
        self.layout = layout;
    }

    pub fn layout(&self) -> &dyn Layout2D {
        self.layout.as_ref()
    }

    pub fn render(&self) {
        self.geometry.borrow().render();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window_manager
            .upgrade()
            .map(|wm| wm.try_borrow_mut().map(|mut wm| wm.remove(self.id())));
    }
}
