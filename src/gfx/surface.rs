pub struct Surface {
    pub(crate) wgpu_surface: wgpu::Surface,
    pub(crate) wgpu_config: wgpu::SurfaceConfiguration,
}

impl Surface {
    pub fn current_frame(&self) -> wgpu::SurfaceTexture {
        self.wgpu_surface.get_current_texture().unwrap()
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.wgpu_config.width = width;
        self.wgpu_config.height = height;
        self.wgpu_surface.configure(device, &self.wgpu_config);
    }
}
