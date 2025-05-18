use std::rc::Rc;

use crate::{
    application::Application,
    core::Result,
    platform::{self, PlatformApplication},
};

use super::{Event, EventHandler};
pub struct EventLoop {
    pub(crate) platform_event_loop: Rc<dyn platform::PlatformEventLoop>,
}

impl EventLoop {
    pub fn new(application: &Application) -> Result<Self> {
        let platform_event_loop =
            platform::EventLoop::new(application.platform_application.clone())?;

        Ok(Self {
            platform_event_loop,
        })
    }

    pub(crate) fn from_platform_application(
        platform_application: Rc<dyn PlatformApplication>,
    ) -> Result<Self> {
        let platform_event_loop = platform::EventLoop::new(platform_application.clone())?;

        Ok(Self {
            platform_event_loop,
        })
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
