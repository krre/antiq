use std::{collections::HashMap, rc::Rc};

use super::{shader::ShaderName, ShaderStorage};

type Pipelines = HashMap<PipelineName, Rc<wgpu::RenderPipeline>>;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum PipelineName {
    Dot,
}

pub struct PipelineStorage {
    pipelines: Pipelines,
}

impl PipelineStorage {
    pub fn new(
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        shader_storage: &ShaderStorage,
    ) -> Self {
        let pipelines = HashMap::from([(
            PipelineName::Dot,
            Self::create_pipeline(
                adapter,
                device,
                surface,
                shader_storage.shader(ShaderName::Dot),
            ),
        )]);

        Self { pipelines }
    }

    pub fn pipeline(&self, name: PipelineName) -> &wgpu::RenderPipeline {
        &self
            .pipelines
            .get(&name)
            .expect(&format!("Pipeline {:?} not found", name))
    }

    fn create_pipeline(
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        shader: &wgpu::ShaderModule,
    ) -> Rc<wgpu::RenderPipeline> {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(swapchain_format.into())],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Rc::new(render_pipeline)
    }
}
