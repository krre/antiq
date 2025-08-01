use crate::ui::d2::geometry::{Pos2D, Size2D};

pub mod event_dispatcher;

pub(crate) use event_dispatcher::EventDispatcher;

pub trait WindowEvent {
    fn resize(&self, size: Size2D) {
        let _ = size;
    }

    fn mouse_move(&self, pos: Pos2D) {
        let _ = pos;
    }
}
