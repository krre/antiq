use crate::platform::PlatformEventLoop;

pub struct EventLoop {}

impl EventLoop {
    pub fn new() -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformEventLoop for EventLoop {
    fn run(&self) {
        println!("Linux Wayland event loop runned");
    }
}
