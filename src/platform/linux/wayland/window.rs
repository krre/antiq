use std::any::Any;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::SurfaceTargetUnsafe;

use crate::{
    core::{Pos2D, Size2D},
    platform::PlatformWindow,
};

pub struct Window {}

struct WaylandWindowHandle {}

impl Window {
    pub fn new() -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn surface_target(&self) -> SurfaceTargetUnsafe {
        let window = WaylandWindowHandle {};

        unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() }
    }

    fn set_title(&self, title: &str) {}

    fn title(&self) -> String {
        String::new()
    }

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
