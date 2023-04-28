use crate::gfx::gpu::Gpu;

use super::{PipelineStorage, ShaderStorage};

pub struct Engine {
    gpu: Gpu,
    _shader_storage: ShaderStorage,
    _pipeline_storage: PipelineStorage,
}

impl Engine {
    pub fn new() -> Self {
        let gpu = Gpu::new();
        let shader_storage = ShaderStorage::new(gpu.device());
        let pipeline_storage = PipelineStorage::new(gpu.device(), &shader_storage);

        Self {
            gpu,
            _shader_storage: shader_storage,
            _pipeline_storage: pipeline_storage,
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
