use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{MouseEvent, Window};

use crate::event::EventHandler;

pub(crate) struct EventDispatcher {
    handlers: Vec<Rc<dyn EventHandler>>,
}

impl EventDispatcher {
    pub(crate) fn new(window: &Window) -> Self {
        // resize
        let resize_closure = Closure::<dyn FnMut()>::new(move || {});
        window
            .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
            .unwrap();
        resize_closure.forget();

        // mousemove
        let mouse_move_closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {});
        window
            .add_event_listener_with_callback(
                "mousemove",
                mouse_move_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_move_closure.forget();

        Self {
            handlers: Vec::new(),
        }
    }

    pub(crate) fn add_handler(&mut self, listener: Rc<dyn EventHandler>) {
        self.handlers.push(listener);
    }
}
