use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{
    core::Result,
    core::event::{Event, EventHandler},
    platform::{PlatformApplication, PlatformEventLoop},
};

use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle};
use wayland_protocols::xdg::shell::client::xdg_surface::{self, XdgSurface};

use super::Application;

pub struct EventLoop {
    application: Rc<dyn PlatformApplication>,
    event_queue: RefCell<EventQueue<State>>,
    pub(crate) queue_handle: QueueHandle<State>,
}

pub(crate) struct State {
    running: bool,
}

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
        let wayland_app = application.as_any().downcast_ref::<Application>().unwrap();
        let wayland_conn = wayland_app.connection.as_ref();
        let event_queue = RefCell::new(wayland_conn.new_event_queue());
        let queue_handle = event_queue.borrow().handle();

        Ok(Rc::new(Self {
            application,
            event_queue,
            queue_handle,
        }))
    }

    fn application(&self) -> &Application {
        self.application
            .as_any()
            .downcast_ref::<Application>()
            .unwrap()
    }

    fn conn(&self) -> &Connection {
        self.application().connection.as_ref()
    }
}

impl PlatformEventLoop for EventLoop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn run(&self, event_handler: &dyn EventHandler) -> Result<()> {
        let mut state = State { running: true };

        while state.running {
            self.event_queue
                .borrow_mut()
                .blocking_dispatch(&mut state)?;
        }

        Ok(())
    }

    fn send_event(&self, event: Box<dyn Event>) {}

    fn quit(&self) {}
}

impl Dispatch<XdgSurface, ()> for State {
    fn event(
        state: &mut Self,
        xdg_surface: &XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("{event:?}");

        if let xdg_surface::Event::Configure { serial, .. } = event {
            xdg_surface.ack_configure(serial);
            // state.surface.attach(Some(&state.buffer), 0, 0);
            // state.surface.commit();
        }
    }
}
