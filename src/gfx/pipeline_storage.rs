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
    pub fn new(device: &wgpu::Device, shader_storage: &ShaderStorage) -> Self {
        let mut pipelines = HashMap::new();
        pipelines.insert(
            PipelineName::Dot,
            Self::create_pipeline(device, shader_storage.shader(ShaderName::Dot)),
        );

        Self { pipelines }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
    ) -> Rc<wgpu::RenderPipeline> {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                // targets: &[Some(swapchain_format.into())],
                targets: &[None],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Rc::new(render_pipeline)
    }
}
