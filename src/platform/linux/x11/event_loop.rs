use std::{any::Any, rc::Rc};

use x11rb::{connection::Connection, protocol};

use crate::{
    core::{
        event::{Event, WindowEvent},
        WindowId,
    },
    platform::{x11::Atoms, PlatformContext, PlatformEventLoop},
};

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

    fn run(&self, event_handler: &dyn Event) -> Result<(), Box<dyn std::error::Error>> {
        let x11_context = self.context.as_any().downcast_ref::<Context>().unwrap();
        let conn = x11_context.connection.as_ref();
        let atoms = Atoms::new(conn)?.reply()?;

        println!("Linux X11 event loop runned");

        loop {
            let event = conn.wait_for_event()?;
            println!("Got event {event:?}");
            match event {
                protocol::Event::Expose(event) => {
                    if event.count == 0 {
                        event_handler.window_event(
                            WindowId::new(event.window as usize),
                            WindowEvent::Redraw,
                        );
                    }
                }
                protocol::Event::ConfigureNotify(event) => {}
                protocol::Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32 && data[0] == atoms.WM_DELETE_WINDOW {
                        event_handler
                            .window_event(WindowId::new(event.window as usize), WindowEvent::Close);
                        break;
                    }
                }
                protocol::Event::Error(err) => {
                    println!("Got an unexpected error: {err:?}")
                }
                event => println!("Got an unhandled event: {event:?}"),
            }
        }

        Ok(())
    }
}
