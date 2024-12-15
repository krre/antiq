use std::{any::Any, rc::Rc};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::COPY_DEPTH_FROM_PARENT;

use crate::platform::{PlatformContext, PlatformWindow};

use super::Context;

pub struct Window {
    context: Rc<dyn PlatformContext>,
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

        let title = "Antiq Window";

        conn.change_property8(
            PropMode::REPLACE,
            win_id,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            title.as_bytes(),
        )?;

        conn.map_window(win_id)?;
        conn.flush()?;

        Ok(Box::new(Self { context: ctx }))
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
