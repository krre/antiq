use std::any::Any;
use std::rc::Rc;

use crate::core::event::EventHandler;
use crate::core::Result;
use crate::platform::{PlatformApplication, PlatformEventLoop};

pub struct EventLoop {}

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
        Ok(Rc::new(Self {}))
    }
}

impl PlatformEventLoop for EventLoop {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn process_events(&self, event_handler: Box<dyn EventHandler>) -> Result<()> {
        Ok(())
    }

    fn send_event(&self, event: Box<dyn crate::core::event::Event>) {
        todo!()
    }

    fn quit(&self) {
        todo!()
    }
}
