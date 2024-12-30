use super::{Pos2D, Size2D, WindowId};

pub enum WindowAction {
    Redraw,
    Close,
    Resize(Size2D),
    Move(Pos2D),
}

pub struct WindowEvent {
    pub id: WindowId,
    pub action: WindowAction,
}

pub trait Event {}

pub trait EventHandler {
    fn window_event(&self, event: WindowEvent) {
        let _ = event;
    }
}

impl Event for WindowEvent {}
