use std::{cell::RefCell, collections::HashMap};

use winit::{event_loop::EventLoopProxy, window::WindowId};

use super::application::UserEvent;

pub struct Context {
    // event_loop_proxy: EventLoopProxy<UserEvent>,
    windows: RefCell<HashMap<WindowId, winit::window::Window>>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            // event_loop_proxy,
            windows: RefCell::new(HashMap::new()),
        }
    }

    // pub(crate) fn event_loop_proxy(&self) -> &EventLoopProxy<UserEvent> {
    //     &self.event_loop_proxy
    // }

    pub(crate) fn windows(&self) -> &RefCell<HashMap<WindowId, winit::window::Window>> {
        &self.windows
    }
}
