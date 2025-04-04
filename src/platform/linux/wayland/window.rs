use std::{any::Any, ffi::c_void, os::fd::AsFd, ptr::NonNull, rc::Rc};

use raw_window_handle::{
    HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle,
    WaylandWindowHandle,
};
use wayland_client::{
    Proxy, delegate_noop,
    protocol::{wl_buffer::WlBuffer, wl_shm, wl_shm_pool::WlShmPool, wl_surface::WlSurface},
};
use wayland_protocols::xdg::{
    decoration::zv1::client::zxdg_toplevel_decoration_v1::{self, ZxdgToplevelDecorationV1},
    shell::client::{xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel},
};
use wgpu::SurfaceTargetUnsafe;

use crate::{
    core::{Pos2D, Size2D},
    platform::{PlatformApplication, PlatformEventLoop, PlatformWindow},
    window::WindowId,
};

use super::{Application, EventLoop, State};

pub struct Window {
    id: WindowId,
    application: Rc<Application>,
    event_loop: Rc<EventLoop>,
    surface: WlSurface,
    xdg_surface: XdgSurface,
    xdg_toplevel: XdgToplevel,
    xdg_toplevel_decoration: ZxdgToplevelDecorationV1,
}
struct WindowHandle {
    surface: *mut c_void,
    display: *mut c_void,
}

#[derive(Debug)]
pub(crate) struct XdgSurfaceData {
    pub window_id: WindowId,
}

#[derive(Debug)]
pub(crate) struct XdgToplevelData {
    pub window_id: WindowId,
}

delegate_noop!(State: ignore WlSurface);
delegate_noop!(State: ignore WlShmPool);
delegate_noop!(State: ignore WlBuffer);
delegate_noop!(State: ignore ZxdgToplevelDecorationV1);

impl Window {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
        event_loop: Rc<dyn PlatformEventLoop>,
        size: Size2D,
    ) -> crate::core::Result<Box<dyn PlatformWindow>> {
        let application = Rc::downcast::<Application>(application.clone() as Rc<dyn Any>).unwrap();
        let event_loop = Rc::downcast::<EventLoop>(event_loop.clone() as Rc<dyn Any>).unwrap();
        let qh = &event_loop.queue_handle;

        let surface = application.compositor.create_surface(qh, ());

        let id = WindowId::generate_new();

        let xdg_surface =
            event_loop
                .xdg_wm_base
                .get_xdg_surface(&surface, qh, XdgSurfaceData { window_id: id });

        let xdg_toplevel = xdg_surface.get_toplevel(qh, XdgToplevelData { window_id: id });

        let xdg_toplevel_decoration =
            application
                .xdg_decoration_manager
                .get_toplevel_decoration(&xdg_toplevel, qh, ());
        xdg_toplevel_decoration.set_mode(zxdg_toplevel_decoration_v1::Mode::ServerSide);

        surface.commit();

        Ok(Box::new(Self {
            id,
            application,
            event_loop,
            surface,
            xdg_surface,
            xdg_toplevel,
            xdg_toplevel_decoration,
        }))
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.surface.destroy();
    }
}

impl PlatformWindow for Window {
    fn id(&self) -> WindowId {
        self.id
    }

    fn surface_target(&self) -> SurfaceTargetUnsafe {
        let window = WindowHandle {
            surface: self.surface.id().as_ptr() as *mut c_void,
            display: self.application.connection.backend().display_ptr() as *mut c_void,
        };

        unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() }
    }

    fn set_title(&self, title: &str) {
        self.xdg_toplevel.set_title(String::from(title));
    }

    fn set_visible(&self, visible: bool) {}

    fn set_position(&self, pos: Pos2D) {}

    fn set_size(&self, size: Size2D) {
        let qh = &self.event_loop.queue_handle;

        let stride = size.width() as i32 * 4;
        let buffer_size = (stride * size.height() as i32) as usize;

        let file = tempfile::tempfile().unwrap();
        file.set_len(buffer_size as u64).unwrap();

        let pool = self
            .application
            .shm
            .create_pool(file.as_fd(), buffer_size as i32, qh, ());

        let buffer = pool.create_buffer(
            0,
            size.width() as i32,
            size.height() as i32,
            stride,
            wl_shm::Format::Argb8888,
            qh,
            (),
        );

        self.surface.attach(Some(&buffer), 0, 0);
        self.surface.commit();
    }
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
