use crate::core::Result;
use crate::platform::PlatformEventLoop;

pub struct EventLoop {}

impl EventLoop {
    pub fn new() -> Result<Box<dyn PlatformEventLoop>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformEventLoop for EventLoop {
    fn process_events(&self, event_handler: Box<dyn EventHandler>) -> Result<()> {
        Ok(())
    }
}
