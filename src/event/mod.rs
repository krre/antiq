use crate::ui::d2::geometry::{Pos2D, Size2D};

pub mod event_dispatcher;

pub(crate) use event_dispatcher::EventDispatcher;

pub enum Event<T> {
    WindowResize(Size2D),
    MouseMove(Pos2D),
    User(T),
}

pub trait EventHandler<T = ()> {
    fn handle(event: Event<T>) {
        let _ = event;
    }
}
