use super::{Pos2D, Size2D, WindowId};

pub enum WindowEvent {
    Redraw,
    Close,
    Resize(Size2D),
    Move(Pos2D),
}

pub trait Event {
    fn window_event(&self, id: WindowId, event: WindowEvent) {}
}
