use std::rc::Rc;

use crate::platform;

use super::{
    application::Application,
    event::{Event, EventHandler},
};

pub struct EventLoop {
    pub(crate) platform_event_loop: Rc<dyn platform::PlatformEventLoop>,
}

struct Dummy;

impl EventLoop {
    pub fn new(application: &Application) -> Result<Self, Box<dyn std::error::Error>> {
        let platform_event_loop =
            platform::EventLoop::new(application.platform_application.clone())?;

        Ok(Self {
            platform_event_loop,
        })
    }

    pub fn new_uninit() -> Self {
        Self {
            platform_event_loop: Rc::new(Dummy {}),
        }
    }

    pub fn run(&self, event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        self.platform_event_loop.run(event_handler)?;
        Ok(())
    }

    pub fn send_event(&self, event: Box<dyn Event>) {
        self.platform_event_loop.send_event(event);
    }

    pub fn quit(&self) {
        self.platform_event_loop.quit();
    }
}

impl platform::PlatformEventLoop for Dummy {
    fn as_any(&self) -> &dyn std::any::Any {
        unimplemented!()
    }

    fn run(&self, _event_handler: &dyn EventHandler) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn send_event(&self, _event: Box<dyn Event>) {
        unimplemented!()
    }

    fn quit(&self) {
        unimplemented!()
    }
}
