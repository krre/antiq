use std::{cell::RefCell, collections::HashMap};

use winit::{event_loop::EventLoopProxy, window::WindowId};

use super::application::UserEvent;

pub struct AppContext {
    event_loop_proxy: Option<EventLoopProxy<UserEvent>>,
    windows: HashMap<WindowId, RefCell<winit::window::Window>>,
}

impl AppContext {
    pub(crate) fn new() -> Self {
        Self {
            event_loop_proxy: None,
            windows: HashMap::new(),
        }
    }

    pub(crate) fn event_loop_proxy(&self) -> &EventLoopProxy<UserEvent> {
        &self.event_loop_proxy.as_ref().unwrap()
    }

    pub(crate) fn set_event_loop_proxy(&mut self, event_loop_proxy: EventLoopProxy<UserEvent>) {
        self.event_loop_proxy = Some(event_loop_proxy);
    }

    pub(crate) fn add_window(&mut self, id: WindowId, window: winit::window::Window) {
        self.windows.insert(id, RefCell::new(window));
    }
}
