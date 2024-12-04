use crate::platform::PlatformEventLoop;

pub struct EventLoop {}

impl EventLoop {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlatformEventLoop for EventLoop {
    fn run(&self) {
        println!("Linux event loop runned");
    }
}
