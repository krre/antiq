use wgpu::{Adapter, Device, Instance, Queue};

pub struct Gpu {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Gpu {
    pub fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        let adapter_options = wgpu::RequestAdapterOptions {
            ..Default::default()
        };

        let adapter = pollster::block_on(instance.request_adapter(&adapter_options)).unwrap();

        println!("Graphics adapter: {}", adapter.get_info().name);

        let device_descriptor = wgpu::DeviceDescriptor::default();
        let device_future = adapter.request_device(&device_descriptor, None);
        let (device, queue) = pollster::block_on(device_future).unwrap();

        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}
