use std::{any::Any, rc::Rc};

use x11rb::{connection::Connection, protocol::Event};

use crate::platform::{x11::Atoms, PlatformContext, PlatformEventLoop};

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
        let atoms = Atoms::new(conn)?.reply()?;

        println!("Linux X11 event loop runned");

        loop {
            let event = conn.wait_for_event()?;
            println!("Got event {event:?}");
            match event {
                Event::Expose(event) => if event.count == 0 {},
                Event::ConfigureNotify(event) => {}
                Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32
                    // && event.window == win_id
                    && data[0] == atoms.WM_DELETE_WINDOW
                    {
                        println!("Window was asked to close");
                        break;
                    }
                }
                Event::Error(err) => {
                    println!("Got an unexpected error: {err:?}")
                }
                event => println!("Got an unhandled event: {event:?}"),
            }
        }

        Ok(())
    }
}
