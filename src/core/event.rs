use super::WindowId;

pub enum WindowEvent {
    Redraw,
    Close,
}

pub trait Event {
    fn window_event(&self, id: WindowId, event: WindowEvent) {}
}
