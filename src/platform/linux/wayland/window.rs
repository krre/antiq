use std::{any::Any, ffi::c_void, fs::File, os::fd::AsFd, ptr::NonNull, rc::Rc};

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

delegate_noop!(State: ignore WlSurface);
delegate_noop!(State: ignore WlShmPool);
delegate_noop!(State: ignore WlBuffer);
delegate_noop!(State: ignore XdgToplevel);
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
        let xdg_surface_data = XdgSurfaceData { window_id: id };

        let xdg_surface =
            wayland_application
                .xdg_wm_base
                .get_xdg_surface(&surface, qh, xdg_surface_data);

        let xdg_toplevel = xdg_surface.get_toplevel(qh, ());

        let xdg_toplevel_decoration = wayland_application
            .xdg_decoration_manager
            .get_toplevel_decoration(&xdg_toplevel, qh, ());
        xdg_toplevel_decoration.set_mode(zxdg_toplevel_decoration_v1::Mode::ServerSide);

        let mut file = tempfile::tempfile().unwrap();
        draw(&mut file, (size.width(), size.height()));

        let pool = wayland_application.shm.create_pool(
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

        surface.attach(Some(&buffer), 0, 0);
        surface.commit();

        Ok(Box::new(Self {
            id,
            application,
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
}

// Temporaty drawing from wayland-rs example
// https://github.com/Smithay/wayland-rs/blob/master/wayland-client/examples/simple_window.rs
fn draw(tmp: &mut File, (buf_x, buf_y): (u32, u32)) {
    use std::{cmp::min, io::Write};
    let mut buf = std::io::BufWriter::new(tmp);
    for y in 0..buf_y {
        for x in 0..buf_x {
            let a = 0xFF;
            let r = min(((buf_x - x) * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
            let g = min((x * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
            let b = min(((buf_x - x) * 0xFF) / buf_x, (y * 0xFF) / buf_y);
            buf.write_all(&[b as u8, g as u8, r as u8, a as u8])
                .unwrap();
        }
    }
    buf.flush().unwrap();
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
