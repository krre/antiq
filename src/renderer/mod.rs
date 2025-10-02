use wasm_bindgen::UnwrapThrowExt;

use crate::{
    renderer::webgpu::{Adapter, Gpu},
    ui::d2::geometry::Size2D,
};

pub mod webgpu;

pub struct Renderer {
    gpu: Gpu,
    adapter: Adapter,
}

impl Renderer {
    pub(crate) async fn new(gpu: Gpu) -> Self {
        let adapter = gpu
            .request_adapter()
            .await
            .expect_throw("Can't request adapter");
        Self { gpu, adapter }
    }

    pub fn resize(&self, size: &Size2D) {
        // gloo::console::log!("resize renderer", size.width(), size.height())
    }

    pub fn render(&self) {}

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
