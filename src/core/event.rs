use std::any::Any;

use crate::ui::d2::geometry::{Pos2D, Size2D};

use super::window::WindowId;
pub enum ApplicationAction {
    Quit,
}

pub enum WindowAction {
    Redraw,
    Close,
    Resize(Size2D),
    AskResize(Size2D),
    Move(Pos2D),
}

pub struct ApplicationEvent {
    pub action: ApplicationAction,
}

pub struct WindowEvent {
    pub id: WindowId,
    pub action: WindowAction,
}

pub trait Event: Any {}

pub trait EventHandler {
    fn window_event(&self, event: WindowEvent) {
        let _ = event;
    }
}
