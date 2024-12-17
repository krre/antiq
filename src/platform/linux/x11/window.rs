use std::{any::Any, rc::Rc};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::COPY_DEPTH_FROM_PARENT;

use crate::{
    core::Pos2D,
    platform::{PlatformContext, PlatformWindow},
};

use super::Context;

pub struct Window {
    context: Rc<dyn PlatformContext>,
    id: u32,
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

    fn conn(&self) -> &RustConnection {
        self.context().connection.as_ref()
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
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
}
