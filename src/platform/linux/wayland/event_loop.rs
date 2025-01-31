use std::{any::Any, rc::Rc};

use crate::{
    core::event::{Event, EventHandler},
    platform::{PlatformApplication, PlatformEventLoop},
};

use wayland_client::{Connection, EventQueue};

use super::Application;

pub struct EventLoop {
    application: Rc<dyn PlatformApplication>,
}

struct State {
    running: bool,
}

impl EventLoop {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
    ) -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self { application }))
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

    fn run(&self, event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = State { running: true };

        let mut event_queue: EventQueue<State> = self.conn().new_event_queue();

        while state.running {
            event_queue.blocking_dispatch(&mut state)?;
        }

        Ok(())
    }

    fn send_event(&self, event: Box<dyn Event>) {}

    fn quit(&self) {}
}
