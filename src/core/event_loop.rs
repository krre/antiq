use std::rc::Rc;

use crate::platform;

use super::{
    event::{Event, EventHandler},
    Context,
};

pub struct EventLoop {
    context: Rc<Context>,
    platform_event_loop: Box<dyn platform::PlatformEventLoop>,
}

impl EventLoop {
    pub fn new(context: Rc<Context>) -> Result<Self, Box<dyn std::error::Error>> {
        let platform_event_loop = platform::EventLoop::new(context.platform_context.clone())?;

        Ok(Self {
            context,
            platform_event_loop,
        })
    }

    pub fn run(&self, event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        self.platform_event_loop.run(event_handler)?;
        Ok(())
    }

    pub fn send_event(&self, event: Box<dyn Event>) {
        self.platform_event_loop.send_event(event);
    }
}
