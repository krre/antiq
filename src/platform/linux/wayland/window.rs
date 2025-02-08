use std::{any::Any, os::fd::AsFd, rc::Rc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wayland_client::{protocol::{wl_buffer::WlBuffer, wl_shm, wl_surface::WlSurface}, Connection};
use wayland_protocols::xdg::shell::client::{xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel};
use wgpu::SurfaceTargetUnsafe;

use crate::{
    core::{Pos2D, Size2D},
    platform::{PlatformApplication, PlatformEventLoop, PlatformWindow},
    window::WindowId,
};

use super::{Application, EventLoop};

pub struct Window {
    application: Rc<dyn PlatformApplication>,
    buffer: WlBuffer,
    surface: WlSurface,
    xdg_surface: XdgSurface,
    xdg_toplevel: XdgToplevel
}

struct WaylandWindowHandle {}

impl Window {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
        event_loop: Rc<dyn PlatformEventLoop>,
        size: Size2D,
    ) -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        let wayland_application = application
        .as_any()
        .downcast_ref::<Application>()
        .unwrap();
        let wayland_event_loop = event_loop.as_any().downcast_ref::<EventLoop>().unwrap();
        let qh = &wayland_event_loop.queue_handle;

        let file = tempfile::tempfile().unwrap();
        let pool = wayland_application.shm.create_pool(file.as_fd(), (size.width() * size.height() * 4) as i32, qh, ());
        let buffer = pool.create_buffer(
            0,
            size.width() as i32,
            size.height() as i32,
            (size.width() * 4) as i32,
            wl_shm::Format::Argb8888,
            qh,
            (),
        );

        let surface = wayland_application.compositor.create_surface(qh, ());
        surface.attach(Some(&buffer), 0, 0);
        surface.commit();

        let xdg_surface = wayland_application.xdg_wm_base.get_xdg_surface(&surface, qh, ());
        let xdg_toplevel = xdg_surface.get_toplevel(qh, ());
        xdg_toplevel.set_title("Wayland window".into());

        surface.commit();

        Ok(Box::new(Self { application, buffer, surface, xdg_surface, xdg_toplevel }))
    }

    fn application(&self) -> &Application {
        self.application
            .as_any()
            .downcast_ref::<Application>()
            .unwrap()
    }

    fn conn(&self) -> &Connection {
        self.application().connection.as_ref()
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
