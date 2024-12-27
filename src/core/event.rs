use super::{Size2D, WindowId};

pub enum WindowEvent {
    Redraw,
    Close,
    Resize(Size2D),
}

pub trait Event {
    fn window_event(&self, id: WindowId, event: WindowEvent) {}
}
