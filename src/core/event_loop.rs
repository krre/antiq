use crate::platform;
pub struct EventLoop {
    platform_event_loop: Box<dyn platform::PlatformEventLoop>,
}

impl EventLoop {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            platform_event_loop: platform::EventLoop::new()?,
        })
    }

    pub fn run(&self) {
        self.platform_event_loop.run();
    }
}
