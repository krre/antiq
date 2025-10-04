use std::rc::Rc;

use futures::StreamExt;
use futures::channel::mpsc::{self, UnboundedSender};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::spawn_local;

use crate::Window;
use crate::event::{Event, EventHandler};
use crate::ui::d2::geometry::Pos2D;

pub(crate) struct EventDispatcher {
    handlers: Vec<Rc<dyn EventHandler>>,
    sender: UnboundedSender<Event<()>>,
    _closures: Vec<Closure<dyn FnMut(web_sys::Event)>>,
}

impl EventDispatcher {
    pub(crate) fn new(handlers: Vec<Rc<dyn EventHandler>>) -> Rc<Self> {
        let window = Window::window();
        let (sender, mut receiver) = mpsc::unbounded::<Event<()>>();
        let mut closures = Vec::new();

        // Resize event
        let resize_sender = sender.clone();
        let resize_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let size = Window::size();
            resize_sender.unbounded_send(Event::WindowResize(size)).ok();
        }) as Box<dyn FnMut(web_sys::Event)>);

        window
            .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
            .expect_throw("Failed to add resize listener");
        closures.push(resize_closure);

        // Mouse move event
        let mouse_move_sender = sender.clone();
        let mouse_move_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let event = event
                .dyn_ref::<web_sys::MouseEvent>()
                .expect_throw("Can't get mouse move event");
            let x = event.client_x();
            let y = event.client_y();
            mouse_move_sender
                .unbounded_send(Event::MouseMove(Pos2D::new(x, y)))
                .ok();
        }) as Box<dyn FnMut(web_sys::Event)>);

        window
            .add_event_listener_with_callback(
                "mousemove",
                mouse_move_closure.as_ref().unchecked_ref(),
            )
            .expect_throw("Failed to add mousemove listener");
        closures.push(mouse_move_closure);

        let dispatcher = Rc::new(Self {
            handlers,
            sender,
            _closures: closures,
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
