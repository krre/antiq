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
        Pos2D, Size2D,
        event::{Event, EventHandler, WindowAction, WindowEvent},
    },
    platform::{PlatformApplication, PlatformEventLoop, x11::Atoms},
    window::WindowId,
};

use super::Application;

pub struct EventLoop {
    application: Rc<dyn PlatformApplication>,
}

impl EventLoop {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
    ) -> Result<Rc<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        let x11_app = application.as_any().downcast_ref::<Application>().unwrap();
        let conn = x11_app.connection.as_ref();
        let screen = conn.setup().roots[x11_app.screen_num].clone();
        conn.change_window_attributes(
            screen.root,
            &ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
        )?;
        conn.flush()?;

        Ok(Rc::new(Self { application }))
    }

    fn application(&self) -> &Application {
        self.application
            .as_any()
            .downcast_ref::<Application>()
            .unwrap()
    }

    fn conn(&self) -> &XCBConnection {
        self.application
            .as_any()
            .downcast_ref::<Application>()
            .unwrap()
            .connection
            .as_ref()
    }

    fn send_client_message_event(
        &self,
        type_: impl Into<Atom>,
        data: impl Into<ClientMessageData>,
    ) {
        let screen = &self.conn().setup().roots[self.application().screen_num];
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

                    if event.type_ == self.application().atoms.QUIT_EVENT {
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
        self.send_client_message_event(self.application().atoms.CLIENT_EVENT, [0, 0, 0, 0, 0]);
    }

    fn quit(&self) {
        self.send_client_message_event(self.application().atoms.QUIT_EVENT, [0, 0, 0, 0, 0]);
    }
}
