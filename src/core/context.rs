use std::{cell::RefCell, collections::HashMap};

use winit::{event_loop::EventLoopProxy, window::WindowId};

use super::application::UserEvent;

pub struct AppContext {
    event_loop_proxy: EventLoopProxy<UserEvent>,
    windows: RefCell<HashMap<WindowId, winit::window::Window>>,
}

impl AppContext {
    pub(crate) fn new(event_loop_proxy: EventLoopProxy<UserEvent>) -> Self {
        Self {
            event_loop_proxy,
            windows: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn event_loop_proxy(&self) -> &EventLoopProxy<UserEvent> {
        &self.event_loop_proxy
    }

    pub(crate) fn add_window(&self, id: WindowId, window: winit::window::Window) {
        self.windows.borrow_mut().insert(id, window);
    }
}
