use wgpu::{Adapter, Device, Instance, Queue};

pub struct Gpu {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Gpu {
    pub fn new() -> Self {
        let instance = wgpu::Instance::default();
        // FIXME: Founded adapter may not match the window surface. Better use instance.request_adapter() with appropriate surface.
        let adapter = Self::find_adapter(&instance);

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
