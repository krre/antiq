use raw_window_handle::{HasDisplayHandle, HasWindowHandle, XcbDisplayHandle, XcbWindowHandle};
use std::{any::Any, ffi::c_void, num::NonZeroU32, ptr::NonNull, rc::Rc};
use wgpu::SurfaceTargetUnsafe;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::xcb_ffi::XCBConnection;
use x11rb::COPY_DEPTH_FROM_PARENT;

use crate::{
    core::{Pos2D, Size2D, WindowId},
    platform::{PlatformContext, PlatformWindow},
};

use super::Context;

pub struct Window {
    context: Rc<dyn PlatformContext>,
    id: u32,
}

struct X11WindowHandle {
    connection: *mut c_void,
    window_id: u32,
    screen_num: i32,
}

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

impl Window {
    pub fn new(
        context: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        let x11_context = context.as_any().downcast_ref::<Context>().unwrap();
        let conn = x11_context.connection.as_ref();
        let screen = &conn.setup().roots[x11_context.screen_num];
        let id = conn.generate_id()?;

        let atoms = Atoms::new(conn)?.reply()?;

        conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            id,
            screen.root,
            0,
            0,
            800,
            600,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new()
                .event_mask(EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY)
                .background_pixel(screen.black_pixel),
        )?;

        conn.change_property32(
            PropMode::REPLACE,
            id,
            atoms.WM_PROTOCOLS,
            AtomEnum::ATOM,
            &[atoms.WM_DELETE_WINDOW],
        )?;

        conn.flush()?;

        Ok(Box::new(Self { context, id }))
    }

    fn context(&self) -> &Context {
        self.context.as_any().downcast_ref::<Context>().unwrap()
    }

    fn conn(&self) -> &XCBConnection {
        self.context().connection.as_ref()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.conn().destroy_window(self.id).unwrap();
        self.conn().flush().unwrap();
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> WindowId {
        WindowId::new(self.id as usize)
    }

    fn surface_target(&self) -> SurfaceTargetUnsafe {
        let connection =
            as_raw_xcb_connection::AsRawXcbConnection::as_raw_xcb_connection(self.conn()) as *mut _;

        let window = X11WindowHandle {
            connection,
            window_id: self.id,
            screen_num: self.context().screen_num as i32,
        };

        unsafe { SurfaceTargetUnsafe::from_window(&window).unwrap() }
    }

    fn set_title(&self, title: &str) {
        self.conn()
            .change_property8(
                PropMode::REPLACE,
                self.id,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                title.as_bytes(),
            )
            .unwrap();

        self.conn().flush().unwrap();
    }

    fn set_visible(&self, visible: bool) {
        if visible {
            self.conn().map_window(self.id).unwrap();
        } else {
            self.conn().unmap_window(self.id).unwrap();
        }

        self.conn().flush().unwrap();
    }

    fn set_position(&self, pos: Pos2D) {
        self.conn()
            .configure_window(self.id, &ConfigureWindowAux::new().x(pos.x()).y(pos.y()))
            .unwrap();
        self.conn().flush().unwrap();
    }

    fn set_size(&self, size: Size2D) {
        self.conn()
            .configure_window(
                self.id,
                &ConfigureWindowAux::new()
                    .width(size.width())
                    .height(size.height()),
            )
            .unwrap();
        self.conn().flush().unwrap();
    }
}

impl HasWindowHandle for X11WindowHandle {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let window_handle = XcbWindowHandle::new(NonZeroU32::new(self.window_id).unwrap());
        let raw_handle = raw_window_handle::RawWindowHandle::Xcb(window_handle);
        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(raw_handle)) }
    }
}

impl HasDisplayHandle for X11WindowHandle {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let display_handle = XcbDisplayHandle::new(NonNull::new(self.connection), self.screen_num);
        let raw_handle = raw_window_handle::RawDisplayHandle::Xcb(display_handle);
        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(raw_handle)) }
    }
}
