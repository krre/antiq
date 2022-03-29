pub struct Gpu {
    _instance: wgpu::Instance,
}

impl Gpu {
    pub fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        Self {
            _instance: instance,
        }
    }
}
