use std::rc::Rc;

use gloo::events::EventListener;
use wasm_bindgen::JsCast;

use crate::event::EventHandler;
use crate::Window;

pub(crate) struct EventDispatcher {
    handlers: Vec<Rc<dyn EventHandler>>,
    _listeners: Vec<EventListener>,
}

impl EventDispatcher {
    pub(crate) fn new() -> Self {
        let window = gloo::utils::window();

        let resize_listener = EventListener::new(&window, "resize", move |_| {
            let size = Window::size();
            gloo::console::log!("resize", size.width(), size.height());
        });

        let mouse_move_listener = EventListener::new(&window, "mousemove", move |event| {
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap();
            let x = event.client_x();
            let y = event.client_y();
            gloo::console::log!("mousemove", x, y);
        });

        Self {
            handlers: Vec::new(),
            _listeners: vec![resize_listener, mouse_move_listener]
        }
    }

    pub(crate) fn add_handler(&mut self, listener: Rc<dyn EventHandler>) {
        self.handlers.push(listener);
    }
}
