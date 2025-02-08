use std::{any::Any, ffi::c_void, os::fd::AsFd, ptr::NonNull, rc::Rc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle};
use wayland_client::{protocol::{wl_buffer::WlBuffer, wl_shm, wl_surface::WlSurface}, Connection, Proxy};
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

struct WindowHandle {
    surface: *mut c_void,
    display: *mut c_void,

}

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
        let window = WindowHandle {
            surface: self.surface.id().as_ptr() as *mut c_void,
            display: self.conn().backend().display_ptr() as *mut c_void
        };

        unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() }
    }

    fn set_title(&self, title: &str) {}

    fn set_visible(&self, visible: bool) {}

    fn set_position(&self, pos: Pos2D) {}

    fn set_size(&self, size: Size2D) {}
}

impl HasWindowHandle for WindowHandle {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let raw_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(self.surface).unwrap(),
        ));

        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(raw_handle)) }
    }
}

impl HasDisplayHandle for WindowHandle {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let raw_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            NonNull::new(self.display).unwrap(),
        ));

        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(raw_handle)) }
    }
}
