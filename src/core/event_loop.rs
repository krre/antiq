use std::rc::Rc;

use crate::platform;

use super::Context;
pub struct EventLoop {
    context: Rc<Context>,
    platform_event_loop: Box<dyn platform::PlatformEventLoop>,
}

impl EventLoop {
    pub fn new(ctx: Rc<Context>) -> Result<Self, Box<dyn std::error::Error>> {
        let platform_event_loop = platform::EventLoop::new(ctx.platform_context.clone())?;

        Ok(Self {
            context: ctx,
            platform_event_loop,
        })
    }

    pub fn run(&self) {
        self.platform_event_loop.run();
    }
}
