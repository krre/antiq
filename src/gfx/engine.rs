use crate::gfx::gpu::Gpu;

use super::ShaderStorage;

pub struct Engine {
    gpu: Gpu,
    shader_storage: ShaderStorage,
}

impl Engine {
    pub fn new() -> Self {
        let gpu = Gpu::new();

        let shader_storage = ShaderStorage::new();
        shader_storage.load(gpu.device());

        Self {
            gpu,
            shader_storage,
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
