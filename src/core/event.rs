use std::any::Any;

use super::window::WindowId;
use super::{Pos2D, Size2D};
pub enum ApplicationAction {
    Quit,
}

pub enum WindowAction {
    Redraw,
    Close,
    Resize(Size2D),
    Move(Pos2D),
}

pub struct ApplicationEvent {
    pub action: ApplicationAction,
}

pub struct WindowEvent {
    pub id: WindowId,
    pub action: WindowAction,
}

pub trait Event: Any {
    fn as_any(&self) -> &dyn Any;
}

pub trait EventHandler {
    fn window_event(&self, event: WindowEvent) {
        let _ = event;
    }
}

impl Event for WindowEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Event for ApplicationEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
