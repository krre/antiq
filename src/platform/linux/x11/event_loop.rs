use std::{any::Any, rc::Rc};

use x11rb::{connection::Connection, protocol};

use crate::{
    core::{
        event::{EventHandler, WindowAction, WindowEvent},
        Pos2D, Size2D, WindowId,
    },
    platform::{x11::Atoms, PlatformContext, PlatformEventLoop},
};

use super::Context;

pub struct EventLoop {
    context: Rc<dyn PlatformContext>,
}

impl EventLoop {
    pub fn new(
        context: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self { context }))
    }
}

impl PlatformEventLoop for EventLoop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn run(&self, event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        let x11_context = self.context.as_any().downcast_ref::<Context>().unwrap();
        let conn = x11_context.connection.as_ref();
        let atoms = Atoms::new(conn)?.reply()?;

        println!("Linux X11 event loop runned");

        let mut prev_window_size = Size2D::new(0, 0);
        let mut prev_window_pos = Pos2D::new(0, 0);

        loop {
            let event = conn.wait_for_event()?;
            println!("Got event {event:?}");
            match event {
                protocol::Event::Expose(event) => {
                    if event.count == 0 {
                        event_handler.window_event(WindowEvent {
                            id: WindowId::new(event.window as usize),
                            action: WindowAction::Redraw,
                        });
                    }
                }
                protocol::Event::ConfigureNotify(event) => {
                    let window_size = Size2D::new(event.width as u32, event.height as u32);

                    if window_size != prev_window_size {
                        event_handler.window_event(WindowEvent {
                            id: WindowId::new(event.window as usize),
                            action: WindowAction::Resize(window_size),
                        });
                        prev_window_size = window_size;
                    }

                    let window_pos = Pos2D::new(event.x as i32, event.y as i32);

                    if window_pos != prev_window_pos {
                        event_handler.window_event(WindowEvent {
                            id: WindowId::new(event.window as usize),
                            action: WindowAction::Move(window_pos),
                        });

                        prev_window_pos = window_pos;
                    }
                }
                protocol::Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32
                        && data[0] == atoms.WM_DELETE_WINDOW
                        && event.type_ == atoms.WM_PROTOCOLS
                    {
                        event_handler.window_event(WindowEvent {
                            id: WindowId::new(event.window as usize),
                            action: WindowAction::Close,
                        });
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
