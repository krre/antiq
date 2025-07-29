use crate::{Window, renderer::webgpu::Gpu};

pub mod webgpu;

pub struct Renderer {
    gpu: Gpu,
}

impl Renderer {
    pub(crate) fn new(window: &Window) -> Self {
        let gpu = Gpu::new(window.inner().navigator().gpu());

        Self { gpu }
    }

    pub fn render(&self) {}

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
