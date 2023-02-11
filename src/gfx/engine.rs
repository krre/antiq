use crate::gfx::gpu::Gpu;

pub struct Engine {
    gpu: Gpu,
}

impl Engine {
    pub fn new() -> Self {
        Self { gpu: Gpu::new() }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
