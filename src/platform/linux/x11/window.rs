use raw_window_handle::{HasDisplayHandle, HasWindowHandle, XcbDisplayHandle, XcbWindowHandle};
use std::{any::Any, num::NonZeroU32, rc::Rc};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::xcb_ffi::XCBConnection;
use x11rb::COPY_DEPTH_FROM_PARENT;

use crate::{
    core::{Pos2D, Size2D},
    platform::{PlatformContext, PlatformWindow},
};

use super::Context;

pub struct Window {
    context: Rc<dyn PlatformContext>,
    id: u32,
}

struct X11WindowHandle {
    window_id: u32,
    screen_num: i32,
}

impl Window {
    pub fn new(
        ctx: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        let x11_context = ctx.as_any().downcast_ref::<Context>().unwrap();
        let conn = x11_context.connection.as_ref();
        let screen = &conn.setup().roots[x11_context.screen_num];
        let win_id = conn.generate_id()?;

        conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            win_id,
            screen.root,
            0,
            0,
            800,
            600,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &CreateWindowAux::new().background_pixel(screen.white_pixel),
        )?;

        conn.flush()?;

        Ok(Box::new(Self {
            context: ctx,
            id: win_id,
        }))
    }

    fn context(&self) -> &Context {
        self.context.as_any().downcast_ref::<Context>().unwrap()
    }

    fn conn(&self) -> &XCBConnection {
        self.context().connection.as_ref()
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn window_handle(&self) -> Box<dyn wgpu::WindowHandle + 'static> {
        Box::new(X11WindowHandle {
            window_id: self.id,
            screen_num: self.context().screen_num as i32,
        })
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

    fn title(&self) -> String {
        let reply = self
            .conn()
            .get_property(false, self.id, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024)
            .unwrap()
            .reply()
            .unwrap();
        String::from_utf8(reply.value).unwrap()
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
        let display_handle = XcbDisplayHandle::new(None, self.screen_num);
        let raw_handle = raw_window_handle::RawDisplayHandle::Xcb(display_handle);
        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(raw_handle)) }
    }
}
