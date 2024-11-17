use crate::renderer::Gpu;

use super::ShaderStorage;

pub struct Engine {
    gpu: Gpu,
    _shader_storage: ShaderStorage,
    // _pipeline_storage: PipelineStorage,
}

impl Engine {
    pub fn new() -> Self {
        let gpu = Gpu::new();

        let shader_storage = ShaderStorage::new(gpu.device());
        // let pipeline_storage = PipelineStorage::new(gpu.adapter(), gpu.device(), &shader_storage);

        Self {
            gpu,
            _shader_storage: shader_storage,
            // _pipeline_storage: pipeline_storage,
        }
    }

    pub fn render(&self) {}

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
