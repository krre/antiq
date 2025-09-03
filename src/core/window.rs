use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::{
    core::canvas::Canvas, event::{Event, EventDispatcher, EventHandler}, renderer::webgpu::{CanvasContext, Gpu}, ui::{d2::geometry::Size2D, Ui3d}, Renderer
};

pub struct Window {
    event_dispatcher: Rc<EventDispatcher>,
    ui: Rc<Ui3d>,
    system_event_handler: Rc<SystemEventHandler>,
    renderer: Renderer,
    canvas: Canvas,
}

impl Window {
    pub fn new(ui: Ui3d) -> Self {
        let window = gloo::utils::window();

        let ui = Rc::new(ui);
        let system_event_handler = Rc::new(SystemEventHandler {});

        let event_dispatcher = EventDispatcher::new(vec![ui.clone(), system_event_handler.clone()]);

        let document = gloo::utils::document();
        let canvas = document
            .get_element_by_id("webgpu-canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let size = Self::size();
        canvas.set_width(size.width());
        canvas.set_height(size.height());

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

    pub fn size() -> Size2D {
        let window = gloo::utils::window();
        let width = window.inner_width().ok().unwrap().as_f64().unwrap_or(0.0) as u32;
        let height = window.inner_height().ok().unwrap().as_f64().unwrap_or(0.0) as u32;
        Size2D::new(width, height)
    }
}

pub(crate) struct SystemEventHandler {}

impl EventHandler for SystemEventHandler {
    fn handle(&self, event: &Event<()>) {
        match event {
            Event::WindowResize(size) => gloo::console::log!("size", size.width(), size.height()),
            Event::MouseMove(pos) => gloo::console::log!("pos", pos.x(), pos.y()),
            Event::User(_) => todo!(),
        }
    }
}
