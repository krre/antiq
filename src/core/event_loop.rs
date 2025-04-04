use std::rc::Rc;

use crate::{core::Result, platform};

use super::{
    application::Application,
    event::{Event, EventHandler},
};

pub struct EventLoop {
    pub(crate) platform_event_loop: Rc<dyn platform::PlatformEventLoop>,
}

struct Dummy;

impl EventLoop {
    pub fn new(application: &Application) -> Result<Self> {
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

    pub fn run(&self, event_handler: Box<dyn EventHandler>) -> Result<()> {
        self.platform_event_loop.process_events(event_handler)?;
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
    fn process_events(&self, _event_handler: Box<dyn EventHandler>) -> Result<()> {
        unimplemented!()
    }

    fn send_event(&self, _event: Box<dyn Event>) {
        unimplemented!()
    }

    fn quit(&self) {
        unimplemented!()
    }
}
