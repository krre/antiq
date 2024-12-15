use std::{any::Any, rc::Rc};

use x11rb::connection::Connection;

use crate::platform::{PlatformContext, PlatformEventLoop};

use super::Context;

pub struct EventLoop {
    context: Rc<dyn PlatformContext>,
}

impl EventLoop {
    pub fn new(
        ctx: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self { context: ctx }))
    }
}

impl PlatformEventLoop for EventLoop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let x11_context = self.context.as_any().downcast_ref::<Context>().unwrap();
        let conn = x11_context.connection.as_ref();

        println!("Linux X11 event loop runned");

        Ok(while let Ok(_) = conn.wait_for_event() {})
    }
}
