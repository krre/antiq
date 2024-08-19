use crate::gfx::Gpu;

use super::{
    renderer::{SceneRenderer, UIRenderer},
    Renderer, ShaderStorage,
};

pub struct Engine {
    gpu: Gpu,
    renderers: Vec<Box<dyn Renderer>>,
    _shader_storage: ShaderStorage,
    // _pipeline_storage: PipelineStorage,
}

impl Engine {
    pub fn new() -> Self {
        let gpu = Gpu::new();

        let renderers: Vec<Box<dyn Renderer>> =
            vec![Box::new(SceneRenderer::new()), Box::new(UIRenderer::new())];

        let shader_storage = ShaderStorage::new(gpu.device());
        // let pipeline_storage = PipelineStorage::new(gpu.adapter(), gpu.device(), &shader_storage);

        Self {
            gpu,
            renderers,
            _shader_storage: shader_storage,
            // _pipeline_storage: pipeline_storage,
        }
    }

    pub fn render(&self) {
        for renderer in &self.renderers {
            renderer.render();
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}
