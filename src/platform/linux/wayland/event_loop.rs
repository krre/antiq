use std::any::Any;

use crate::{
    core::event::{Event, EventHandler},
    platform::PlatformEventLoop,
};

pub struct EventLoop {}

impl EventLoop {
    pub fn new() -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformEventLoop for EventLoop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn run(&self, event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn send_event(&self, event: Box<dyn Event>) {}

    fn quit(&self) {}
}
