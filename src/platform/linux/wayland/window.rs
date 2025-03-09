use std::{any::Any, ffi::c_void, os::fd::AsFd, ptr::NonNull, rc::Rc};

use raw_window_handle::{
    HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle,
    WaylandWindowHandle,
};
use wayland_client::{
    Connection, Proxy, delegate_noop,
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
    application: Rc<dyn PlatformApplication>,
    event_loop: Rc<dyn PlatformEventLoop>,
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
        let wayland_application = application.as_any().downcast_ref::<Application>().unwrap();
        let wayland_event_loop = event_loop.as_any().downcast_ref::<EventLoop>().unwrap();
        let qh = &wayland_event_loop.queue_handle;

        let surface = wayland_application.compositor.create_surface(qh, ());

        let id = WindowId::generate_new();

        let xdg_surface = wayland_event_loop.xdg_wm_base.get_xdg_surface(
            &surface,
            qh,
            XdgSurfaceData { window_id: id },
        );

        let xdg_toplevel = xdg_surface.get_toplevel(qh, XdgToplevelData { window_id: id });

        let xdg_toplevel_decoration = wayland_application
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

    fn application(&self) -> &Application {
        self.application
            .as_any()
            .downcast_ref::<Application>()
            .unwrap()
    }

    fn conn(&self) -> &Connection {
        self.application().connection.as_ref()
    }

    fn event_loop(&self) -> &EventLoop {
        self.event_loop
            .as_any()
            .downcast_ref::<EventLoop>()
            .unwrap()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.surface.destroy();
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> WindowId {
        self.id
    }

    fn surface_target(&self) -> SurfaceTargetUnsafe {
        let window = WindowHandle {
            surface: self.surface.id().as_ptr() as *mut c_void,
            display: self.conn().backend().display_ptr() as *mut c_void,
        };

        unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() }
    }

    fn set_title(&self, title: &str) {
        self.xdg_toplevel.set_title(String::from(title));
    }

    fn set_visible(&self, visible: bool) {}

    fn set_position(&self, pos: Pos2D) {}

    fn set_size(&self, size: Size2D) {
        let qh = &self.event_loop().queue_handle;
        let file = tempfile::tempfile().unwrap();

        let pool = self.application().shm.create_pool(
            file.as_fd(),
            (size.width() * size.height() * 4) as i32,
            qh,
            (),
        );

        let buffer = pool.create_buffer(
            0,
            size.width() as i32,
            size.height() as i32,
            (size.width() * 4) as i32,
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
