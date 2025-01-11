use std::{any::Any, rc::Rc};

use x11rb::{
    connection::Connection,
    protocol::{
        self,
        xproto::{
            Atom, ChangeWindowAttributesAux, ClientMessageData, ClientMessageEvent, ConnectionExt,
            EventMask,
        },
    },
    xcb_ffi::XCBConnection,
};

use crate::{
    core::{
        event::{Event, EventHandler, WindowAction, WindowEvent},
        Pos2D, Size2D,
    },
    platform::{x11::Atoms, PlatformContext, PlatformEventLoop},
    window::WindowId,
};

use super::Context;

pub struct EventLoop {
    context: Rc<dyn PlatformContext>,
    quit_atom: u32,
    client_atom: u32,
}

impl EventLoop {
    pub fn new(
        context: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        let x11_context = context.as_any().downcast_ref::<Context>().unwrap();
        let conn = x11_context.connection.as_ref();
        let quit_atom = conn.intern_atom(false, b"QUIT_EVENT")?.reply()?.atom;
        let client_atom = conn.intern_atom(false, b"CLIENT_EVENT")?.reply()?.atom;

        let screen = conn.setup().roots[x11_context.screen_num].clone();
        conn.change_window_attributes(
            screen.root,
            &ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
        )?;
        conn.flush()?;

        Ok(Box::new(Self {
            context,
            quit_atom,
            client_atom,
        }))
    }

    fn context(&self) -> &Context {
        self.context.as_any().downcast_ref::<Context>().unwrap()
    }

    fn conn(&self) -> &XCBConnection {
        self.context().connection.as_ref()
    }

    fn send_client_message_event(
        &self,
        type_: impl Into<Atom>,
        data: impl Into<ClientMessageData>,
    ) {
        let screen = &self.conn().setup().roots[self.context().screen_num];
        let quit_event = ClientMessageEvent::new(32, screen.root, type_, data);

        self.conn()
            .send_event(false, screen.root, EventMask::PROPERTY_CHANGE, quit_event)
            .unwrap();
        self.conn().flush().unwrap();
    }
}

impl PlatformEventLoop for EventLoop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn run(&self, event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn();
        let atoms = Atoms::new(conn)?.reply()?;

        let mut prev_window_size = Size2D::new(0, 0);
        let mut prev_window_pos = Pos2D::new(0, 0);

        loop {
            let event = conn.wait_for_event()?;
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

                    if event.type_ == self.quit_atom {
                        break;
                    }
                }
                protocol::Event::Error(err) => {
                    println!("Unexpected error: {err:?}")
                }
                _ => {} // event => println!("Unhandled event: {event:?}"),
            }
        }

        Ok(())
    }

    fn send_event(&self, event: Box<dyn Event>) {
        self.send_client_message_event(self.client_atom, [0, 0, 0, 0, 0]);
    }

    fn quit(&self) {
        self.send_client_message_event(self.quit_atom, [0, 0, 0, 0, 0]);
    }
}
