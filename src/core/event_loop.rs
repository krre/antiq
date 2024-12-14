use std::rc::Rc;

use crate::platform;

use super::Context;
pub struct EventLoop {
    context: Rc<Context>,
    platform_event_loop: Box<dyn platform::PlatformEventLoop>,
}

impl EventLoop {
    pub fn new(ctx: Rc<Context>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            context: ctx,
            platform_event_loop: platform::EventLoop::new()?,
        })
    }

    pub fn run(&self) {
        self.platform_event_loop.run();
    }
}
