use crate::gfx::gpu::Gpu;

use super::ShaderStorage;

pub struct Engine {
    gpu: Gpu,
    shader_storage: ShaderStorage,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            gpu: Gpu::new(),
            shader_storage: ShaderStorage::new(),
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
