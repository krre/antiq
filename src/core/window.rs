use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{GpuCanvasContext, HtmlCanvasElement};

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
    _event_dispatcher: Rc<EventDispatcher>,
    ui: Rc<Ui3d>,
    _system_event_handler: Rc<SystemEventHandler>,
    renderer: Rc<Renderer>,
}

impl Window {
    pub async fn new(ui: Ui3d) -> Result<Self, JsValue> {
        let window = Self::window();
        let document = window.document().ok_or("Document not found")?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;

        canvas.set_attribute("style", "display: block; width: 100vw; height: 100vh;")?;

        let body = document.body().ok_or("Body element not found")?;
        body.set_attribute("style", "margin: 0; padding: 0;")?;
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
            _event_dispatcher: event_dispatcher,
            ui,
            _system_event_handler: system_event_handler,
            renderer,
        })
    }

    pub fn size() -> Size2D {
        let window = Self::window();
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

    pub fn window() -> web_sys::Window {
        web_sys::window().expect_throw("Global window not found")
    }

    pub fn render(&self) {
        self.renderer.render(&self.ui);
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
        // web_sys::console::log_1(&format!("resize {:?}", size).into());
    }

    fn mouse_move(&self, _pos: &Pos2D) {
        // web_sys::console::log_1(&format!("mouse_move {:?}", pos).into());
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
