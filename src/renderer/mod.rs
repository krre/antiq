mod pipeline_storage;
mod shader;
mod surface;

pub use pipeline_storage::PipelineStorage;
pub use shader::ShaderStorage;
pub use surface::Surface;
use wgpu::{Adapter, Device, Instance, Queue, SurfaceTarget};

pub struct Renderer {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    _shader_storage: ShaderStorage,
    // _pipeline_storage: PipelineStorage,
}

impl Renderer {
    pub fn new() -> Self {
        let instance = wgpu::Instance::default();
        // FIXME: Founded adapter may not match the window surface. Better use instance.request_adapter() with appropriate surface.
        let adapter = Self::find_adapter(&instance);

        log::info!("Graphics adapter: {}", adapter.get_info().name);

        let device_descriptor = wgpu::DeviceDescriptor::default();
        let device_future = adapter.request_device(&device_descriptor, None);
        let (device, queue) = pollster::block_on(device_future).unwrap();

        let shader_storage = ShaderStorage::new(&device);
        // let pipeline_storage = PipelineStorage::new(gpu.adapter(), gpu.device(), &shader_storage);

        Self {
            instance,
            adapter,
            device,
            queue,
            _shader_storage: shader_storage,
            // _pipeline_storage: pipeline_storage,
        }
    }

    pub fn render(&self) {}

    fn find_adapter(instance: &Instance) -> Adapter {
        for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
            if adapter.get_info().device_type == wgpu::DeviceType::DiscreteGpu {
                return adapter;
            }
        }

        let adapter_options = wgpu::RequestAdapterOptions {
            ..Default::default()
        };

        return pollster::block_on(instance.request_adapter(&adapter_options)).unwrap();
    }

    pub fn create_surface<'a>(&self, window: impl Into<SurfaceTarget<'a>>) -> Surface<'a> {
        let wgpu_surface = self.instance.create_surface(window).unwrap();

        let wgpu_config = wgpu_surface
            .get_default_config(&self.adapter, 800, 600)
            .unwrap();

        wgpu_surface.configure(&self.device, &wgpu_config);

        Surface {
            wgpu_surface,
            wgpu_config,
        }
    }

    pub fn clear_view(&self, view: &wgpu::TextureView, color: &wgpu::Color) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(*color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}
