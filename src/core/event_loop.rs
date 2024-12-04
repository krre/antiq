use crate::platform;
pub struct EventLoop {
    inner: Box<dyn platform::PlatformEventLoop>,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            inner: Box::new(platform::EventLoop::new()),
        }
    }

    pub fn run(&self) {
        self.inner.run();
    }
}
