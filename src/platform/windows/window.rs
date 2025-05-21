use std::rc::Rc;

use crate::core::{Result};
use crate::platform::{PlatformApplication, PlatformEventLoop, PlatformWindow};
use crate::ui::d2::geometry::{Pos2D, Size2D};

pub struct Window {}

impl Window {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
        event_loop: Rc<dyn PlatformEventLoop>,
        size: Size2D,
    ) -> Result<Rc<dyn PlatformWindow>> {
        Ok(Rc::new(Self {}))
    }
}

impl PlatformWindow for Window {
    fn id(&self) -> crate::window::WindowId {
        todo!()
    }

    fn surface_target(&self) -> wgpu::SurfaceTargetUnsafe {
        todo!()
    }

    fn set_title(&self, title: &str) {
        todo!()
    }

    fn set_visible(&self, visible: bool) {
        todo!()
    }

    fn set_position(&self, pos: Pos2D) {
        todo!()
    }

    fn set_size(&self, size: Size2D) {
        todo!()
    }
}

pub fn new_window(
    application: Rc<dyn PlatformApplication>,
    event_loop: Rc<dyn PlatformEventLoop>,
    size: Size2D,
) -> Result<Rc<dyn PlatformWindow>> {
    Window::new(application, event_loop, size)
}