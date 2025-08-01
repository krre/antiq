use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{MouseEvent, Window};

pub(crate) struct EventDispatcher {
    resize_closure: Closure<dyn FnMut()>,
    mouse_move_closure: Closure<dyn FnMut(MouseEvent)>,
}

impl EventDispatcher {
    pub(crate) fn new(window: &Window) -> Self {
        // resize
        let resize_closure = Closure::<dyn FnMut()>::new(move || {});

        window
            .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
            .unwrap();

        // mousemove
        let mouse_move_closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {});

        window
            .add_event_listener_with_callback(
                "mousemove",
                mouse_move_closure.as_ref().unchecked_ref(),
            )
            .unwrap();

        Self {
            resize_closure,
            mouse_move_closure,
        }
    }
}
