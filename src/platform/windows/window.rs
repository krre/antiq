use std::any::Any;
use std::rc::Rc;

use crate::core::{Result, Size2D};
use crate::platform::{PlatformApplication, PlatformEventLoop, PlatformWindow};

pub struct Window {}

impl Window {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
        event_loop: Rc<dyn PlatformEventLoop>,
        size: Size2D,
    ) -> Result<Box<dyn PlatformWindow>> {
        Ok(Box::new(Self {}))
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

    fn set_position(&self, pos: crate::core::Pos2D) {
        todo!()
    }

    fn set_size(&self, size: crate::core::Size2D) {
        todo!()
    }
}
