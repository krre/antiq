use std::rc::Rc;

use windows::Win32::UI::WindowsAndMessaging::*;

use crate::core::Result;
use crate::event::{Event, EventHandler};
use crate::platform::{PlatformApplication, PlatformEventLoop};

pub struct EventLoop {}

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
        Ok(Rc::new(Self {}))
    }
}

impl PlatformEventLoop for EventLoop {
    fn process_events(&self, event_handler: Box<dyn EventHandler>) -> Result<()> {
        let mut message = MSG::default();

        unsafe {
            while GetMessageW(&mut message, None, 0, 0).into() {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }

        Ok(())
    }

    fn send_event(&self, event: Box<dyn Event>) {
        todo!()
    }

    fn quit(&self) {
        todo!()
    }
}

pub fn new_event_loop(
    application: Rc<dyn PlatformApplication>,
) -> Result<Rc<dyn PlatformEventLoop>> {
    EventLoop::new(application)
}