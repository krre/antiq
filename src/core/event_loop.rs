use crate::platform;
pub struct EventLoop {
    platform_event_loop: Box<dyn platform::PlatformEventLoop>,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            platform_event_loop: Box::new(platform::EventLoop::new()),
        }
    }

    pub fn run(&self) {
        self.platform_event_loop.run();
    }
}
