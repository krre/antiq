use std::{any::Any, rc::Rc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wayland_client::Connection;
use wgpu::SurfaceTargetUnsafe;

use crate::{
    core::{Pos2D, Size2D},
    platform::{PlatformContext, PlatformWindow},
    window::WindowId,
};

use super::Context;

pub struct Window {
    context: Rc<dyn PlatformContext>,
}

struct WaylandWindowHandle {}

impl Window {
    pub fn new(
        context: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self { context }))
    }

    fn context(&self) -> &Context {
        self.context.as_any().downcast_ref::<Context>().unwrap()
    }

    fn conn(&self) -> &Connection {
        self.context().connection.as_ref()
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> WindowId {
        WindowId::new(0)
    }

    fn surface_target(&self) -> SurfaceTargetUnsafe {
        let window = WaylandWindowHandle {};

        unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() }
    }

    fn set_title(&self, title: &str) {}

    fn set_visible(&self, visible: bool) {}

    fn set_position(&self, pos: Pos2D) {}

    fn set_size(&self, size: Size2D) {}
}

impl HasWindowHandle for WaylandWindowHandle {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        todo!()
    }
}

impl HasDisplayHandle for WaylandWindowHandle {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        todo!()
    }
}
