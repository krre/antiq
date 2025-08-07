use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::{
    Renderer,
    core::canvas::Canvas,
    event::{EventDispatcher, EventHandler},
    renderer::webgpu::{CanvasContext, Gpu},
    ui::{Ui3d, d2::geometry::Size2D},
};

pub struct Window {
    inner: web_sys::Window,
    event_dispatcher: EventDispatcher,
    ui: Rc<Ui3d>,
    system_event_handler: Rc<SystemEventHandler>,
    renderer: Renderer,
    canvas: Canvas,
}

impl Window {
    pub fn new(ui: Ui3d) -> Self {
        let window = web_sys::window().unwrap();

        let ui = Rc::new(ui);
        let system_event_handler = Rc::new(SystemEventHandler {});

        let mut event_dispatcher = EventDispatcher::new(&window);
        event_dispatcher.add_handler(ui.clone());
        event_dispatcher.add_handler(system_event_handler.clone());

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

        let web_sys_context = canvas
            .get_context("webgpu")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::GpuCanvasContext>()
            .unwrap();

        let _webgpu_context = CanvasContext::new(web_sys_context);

        let gpu = Gpu::new(window.navigator().gpu());
        let renderer = Renderer::new(gpu);

        Self {
            inner: window,
            event_dispatcher,
            ui,
            system_event_handler,
            renderer,
            canvas: Canvas::new(canvas),
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
}

pub(crate) struct SystemEventHandler {}

impl EventHandler for SystemEventHandler {}
