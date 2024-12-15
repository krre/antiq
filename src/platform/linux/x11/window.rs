use std::any::Any;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;

use crate::platform::{PlatformContext, PlatformWindow};

use super::Context;

pub struct Window {}

impl Window {
    pub fn new(
        ctx: &dyn PlatformContext,
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
        conn.map_window(win_id)?;
        conn.flush()?;

        Ok(Box::new(Self {}))
    }
}

impl PlatformWindow for Window {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
