use crate::{renderer::webgpu::Gpu, ui::d2::geometry::Size2D};

pub mod webgpu;

pub struct Renderer {
    gpu: Gpu,
}

impl Renderer {
    pub(crate) fn new(gpu: Gpu) -> Self {
        Self { gpu }
    }

    pub fn resize(&self, size: &Size2D) {
        gloo::console::log!("resize renderer", size.width(), size.height())
    }

    pub fn render(&self) {}

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
