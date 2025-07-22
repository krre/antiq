use std::rc::Rc;

use wasm_bindgen::{JsCast, prelude::Closure};

use crate::ui::d2::geometry::Size2D;

pub trait WindowEvent {
    fn resize(&self, size: Size2D) {
        let _ = size;
    }
}

pub struct Window {
    inner: web_sys::Window,
    event_handler: Option<Rc<dyn WindowEvent>>,
    resize_listener: Option<Closure<dyn FnMut()>>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            inner: web_sys::window().unwrap(),
            event_handler: None,
            resize_listener: None,
        }
    }

    pub fn size(&self) -> Size2D {
        let width = self.inner.inner_width().ok().unwrap().as_f64().unwrap() as u32;
        let height = self.inner.inner_height().ok().unwrap().as_f64().unwrap() as u32;
        Size2D::new(width, height)
    }

    pub fn set_event_handler(&mut self, event_hander: Option<Rc<dyn WindowEvent>>) {
        if let Some(closure) = &self.resize_listener {
            let _ = self
                .inner
                .remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
        }

        self.resize_listener = None;
        self.event_handler = None;

        if let Some(hander) = event_hander {
            self.event_handler = Some(hander.clone());
            let window = self.inner.clone();

            let closure = Closure::<dyn FnMut()>::new(move || {
                let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
                let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
                hander.resize(Size2D::new(width, height));
            });

            self.inner
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();

            self.resize_listener = Some(closure);
        }
    }
}
