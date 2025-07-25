use std::rc::Rc;

use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{HtmlCanvasElement, MouseEvent};

use crate::{
    core::canvas::Canvas,
    ui::d2::geometry::{Pos2D, Size2D},
};

pub trait WindowEvent {
    fn resize(&self, size: Size2D) {
        let _ = size;
    }

    fn mouse_move(&self, pos: Pos2D) {
        let _ = pos;
    }
}

pub struct Window {
    inner: web_sys::Window,
    canvas: Canvas,
    event_handler: Option<Rc<dyn WindowEvent>>,
    resize_closure: Option<Closure<dyn FnMut()>>,
    mouse_move_closure: Option<Closure<dyn FnMut(MouseEvent)>>,
}

impl Window {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("webgpu-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        canvas.set_width(width);
        canvas.set_height(height);

        Self {
            inner: window,
            canvas: Canvas::new(canvas),
            event_handler: None,
            resize_closure: None,
            mouse_move_closure: None,
        }
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn size(&self) -> Size2D {
        let width = self.inner.inner_width().ok().unwrap().as_f64().unwrap() as u32;
        let height = self.inner.inner_height().ok().unwrap().as_f64().unwrap() as u32;
        Size2D::new(width, height)
    }

    pub fn set_event_handler(&mut self, event_handler: Option<Rc<dyn WindowEvent>>) {
        if let Some(closure) = &self.resize_closure {
            let _ = self
                .inner
                .remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
        }

        if let Some(mouse_move) = &self.mouse_move_closure {
            let _ = self.inner.remove_event_listener_with_callback(
                "mousemove",
                mouse_move.as_ref().unchecked_ref(),
            );
        }

        self.event_handler = None;

        self.resize_closure = None;
        self.mouse_move_closure = None;

        if let Some(handler) = event_handler {
            self.event_handler = Some(handler.clone());
            let window = self.inner.clone();
            let canvas = self.canvas.inner().clone();
            let resize_handler = handler.clone();

            // resize
            let resize_closure = Closure::<dyn FnMut()>::new(move || {
                let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
                let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
                resize_handler.resize(Size2D::new(width, height));

                canvas.set_width(width);
                canvas.set_height(height);
            });

            self.inner
                .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
                .unwrap();

            self.resize_closure = Some(resize_closure);

            // mousemove
            let mouse_handler = handler.clone();
            let mouse_move_closure =
                Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
                    let x = event.client_x();
                    let y = event.client_y();
                    mouse_handler.mouse_move(Pos2D::new(x, y));
                });

            self.inner
                .add_event_listener_with_callback(
                    "mousemove",
                    mouse_move_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            self.mouse_move_closure = Some(mouse_move_closure);
        }
    }
}
