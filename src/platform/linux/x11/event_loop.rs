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
};

use crate::{
    core::{
        Result, Size2D,
        event::{Event, EventHandler, WindowAction, WindowEvent},
    },
    platform::{PlatformApplication, PlatformEventLoop, x11::Atoms},
    ui::d2::geometry::Pos2D,
    window::WindowId,
};

use super::Application;

pub struct EventLoop {
    application: Rc<Application>,
}

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
        let application = Rc::downcast::<Application>(application.clone() as Rc<dyn Any>).unwrap();
        let conn = application.connection.as_ref();
        let screen = conn.setup().roots[application.screen_num].clone();
        conn.change_window_attributes(
            screen.root,
            &ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
        )?;
        conn.flush()?;

        Ok(Rc::new(Self { application }))
    }

    fn send_client_message_event(
        &self,
        type_: impl Into<Atom>,
        data: impl Into<ClientMessageData>,
    ) {
        let screen = &self.application.connection.setup().roots[self.application.screen_num];
        let quit_event = ClientMessageEvent::new(32, screen.root, type_, data);

        self.application
            .connection
            .send_event(false, screen.root, EventMask::PROPERTY_CHANGE, quit_event)
            .unwrap();
        self.application.connection.flush().unwrap();
    }
}

impl PlatformEventLoop for EventLoop {
    fn process_events(&self, event_handler: Box<dyn EventHandler>) -> Result<()> {
        let conn = &self.application.connection;
        let atoms = Atoms::new(conn)?.reply()?;

        let mut prev_window_size = Size2D::default();
        let mut prev_window_pos = Pos2D::default();

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
                    let window_size = Size2D::new(event.width.into(), event.height.into());

                    if window_size != prev_window_size {
                        event_handler.window_event(WindowEvent {
                            id: WindowId::new(event.window as usize),
                            action: WindowAction::Resize(window_size),
                        });
                        prev_window_size = window_size;
                    } else {
                        let window_pos = Pos2D::new(event.x.into(), event.y.into());

                        if window_pos != prev_window_pos {
                            event_handler.window_event(WindowEvent {
                                id: WindowId::new(event.window as usize),
                                action: WindowAction::Move(window_pos),
                            });

                            prev_window_pos = window_pos;
                        }
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

                    if event.type_ == self.application.atoms.QUIT_EVENT {
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

    fn send_event(&self, _event: Box<dyn Event>) {
        self.send_client_message_event(self.application.atoms.CLIENT_EVENT, [0, 0, 0, 0, 0]);
    }

    fn quit(&self) {
        self.send_client_message_event(self.application.atoms.QUIT_EVENT, [0, 0, 0, 0, 0]);
    }
}
