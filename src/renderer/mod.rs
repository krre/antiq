use crate::renderer::webgpu::Gpu;

pub mod webgpu;

pub struct Renderer {
    gpu: Gpu,
}

impl Renderer {
    pub(crate) fn new(gpu: Gpu) -> Self {
        Self { gpu }
    }

    pub fn render(&self) {}

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
