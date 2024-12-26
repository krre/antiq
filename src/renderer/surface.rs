use crate::{core::Size2D, platform::PlatformWindow};

use super::Renderer;

pub struct Surface {
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) config: wgpu::SurfaceConfiguration,
}

impl Surface {
    pub fn new(window: &dyn PlatformWindow, renderer: &Renderer) -> Self {
        let surface = unsafe {
            renderer
                .instance()
                .create_surface_unsafe(window.surface_target())
                .unwrap()
        };

        let config = surface
            .get_default_config(&renderer.adapter(), 800, 600)
            .expect("Surface isn't supported by the adapter.");

        surface.configure(&renderer.device(), &config);

        Self { surface, config }
    }

    pub fn current_frame(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }

    pub fn set_size(&mut self, device: &wgpu::Device, size: Size2D) {
        self.config.width = size.width();
        self.config.height = size.height();
        self.surface.configure(device, &self.config);
    }
}
