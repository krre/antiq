use std::rc::Rc;

use futures::StreamExt;
use futures::channel::mpsc::{self, UnboundedSender};
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::Window;
use crate::event::{Event, EventHandler};
use crate::ui::d2::geometry::Pos2D;

pub(crate) struct EventDispatcher {
    handlers: Vec<Rc<dyn EventHandler>>,
    sender: UnboundedSender<Event<()>>,
    _listeners: Vec<EventListener>,
}

impl EventDispatcher {
    pub(crate) fn new(handlers: Vec<Rc<dyn EventHandler>>) -> Rc<Self> {
        let window = gloo::utils::window();
        let (sender, mut receiver) = mpsc::unbounded::<Event<()>>();

        let resize_sender = sender.clone();
        let resize_listener = EventListener::new(&window, "resize", move |_| {
            let size = Window::size();
            resize_sender.unbounded_send(Event::WindowResize(size)).ok();
        });

        let mouse_move_sender = sender.clone();
        let mouse_move_listener = EventListener::new(&window, "mousemove", move |event| {
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap();
            let x = event.client_x();
            let y = event.client_y();
            mouse_move_sender
                .unbounded_send(Event::MouseMove(Pos2D::new(x, y)))
                .ok();
        });

        let dispatcher = Rc::new(Self {
            handlers,
            sender,
            _listeners: vec![resize_listener, mouse_move_listener],
        });

        let dispatcher_clone = dispatcher.clone();

        spawn_local(async move {
            while let Some(event) = receiver.next().await {
                for handler in &dispatcher_clone.handlers {
                    handler.handle(&event);
                }
            }
        });

        dispatcher
    }

    pub fn send(&self, event: Event<()>) {
        self.sender.unbounded_send(event).ok();
    }
}
