use wasm_bindgen::JsValue;

use crate::{
    renderer::webgpu::{Adapter, CanvasContext, Device, Gpu},
    ui::d2::geometry::Size2D,
};

pub mod webgpu;

pub struct Renderer {
    gpu: Gpu,
    context: CanvasContext,
    adapter: Adapter,
    device: Device,
}

impl Renderer {
    pub(crate) async fn new(gpu: Gpu, context: CanvasContext) -> Result<Self, JsValue> {
        let adapter = gpu.request_adapter().await?;
        let device = adapter.request_device().await?;

        Ok(Self {
            gpu,
            context,
            adapter,
            device,
        })
    }

    pub fn resize(&self, size: &Size2D) {
        // gloo::console::log!("resize renderer", size.width(), size.height())
    }

    pub fn render(&self) {}

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
