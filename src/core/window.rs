use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{GpuCanvasContext, window};

use crate::{
    Renderer,
    event::{Event, EventDispatcher, EventHandler},
    renderer::webgpu::{CanvasContext, Gpu},
    ui::{
        Ui3d,
        d2::geometry::{Pos2D, Size2D},
    },
};

pub struct Window {
    event_dispatcher: Rc<EventDispatcher>,
    ui: Rc<Ui3d>,
    system_event_handler: Rc<SystemEventHandler>,
    renderer: Rc<Renderer>,
}

impl Window {
    pub async fn new(ui: Ui3d) -> Result<Self, JsValue> {
        let window = window().ok_or("Global window not found")?;

        let body = gloo::utils::body();
        body.set_attribute("style", "margin: 0; padding: 0;")?;

        let document = window.document().ok_or("Document not found")?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        canvas.set_attribute("style", "display: block; width: 100vw; height: 100vh;")?;

        let size = Self::size();
        canvas.set_width(size.width());
        canvas.set_height(size.height());

        body.append_child(&canvas)?;

        let context = canvas
            .get_context("webgpu")?
            .ok_or("WebGPU context not found")?
            .dyn_into::<GpuCanvasContext>()?;

        let gpu = Gpu::new(window.navigator().gpu());
        let context = CanvasContext::new(context);
        let renderer = Rc::new(Renderer::new(gpu, context).await?);

        let system_event_handler = Rc::new(SystemEventHandler::new(renderer.clone()));
        let ui = Rc::new(ui);
        let event_dispatcher = EventDispatcher::new(vec![ui.clone(), system_event_handler.clone()]);

        Ok(Self {
            event_dispatcher,
            ui,
            system_event_handler,
            renderer,
        })
    }

    pub fn size() -> Size2D {
        let window = gloo::utils::window();
        let width = window
            .inner_width()
            .expect_throw("Can't get window width")
            .as_f64()
            .unwrap_or(0.0) as u32;
        let height = window
            .inner_height()
            .expect_throw("Can't get window height")
            .as_f64()
            .unwrap_or(0.0) as u32;
        Size2D::new(width, height)
    }
}

pub(crate) struct SystemEventHandler {
    renderer: Rc<Renderer>,
}

impl SystemEventHandler {
    fn new(renderer: Rc<Renderer>) -> Self {
        SystemEventHandler { renderer }
    }
}

impl SystemEventHandler {
    fn resize(&self, size: &Size2D) {
        self.renderer.resize(size);
    }

    fn mouse_move(&self, pos: &Pos2D) {
        // gloo::console::log!("pos", pos.x(), pos.y())
    }
}

impl EventHandler for SystemEventHandler {
    fn handle(&self, event: &Event<()>) {
        match event {
            Event::WindowResize(size) => self.resize(size),
            Event::MouseMove(pos) => self.mouse_move(pos),
            Event::User(_) => todo!(),
        }
    }
}
