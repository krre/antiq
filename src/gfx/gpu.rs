use winit::{event::WindowEvent, window::Window};

pub struct Gpu {
    // surface: wgpu::Surface,
// device: wgpu::Device,
// queue: wgpu::Queue,
// config: wgpu::SurfaceConfiguration,
// size: winit::dpi::PhysicalSize<u32>,
}

impl Gpu {
    async fn new(window: &Window) -> Self {
        // let size = window.inner_size();
        Self {}

        // let instance = wgpu::Instance::new(wgpu::Backends::all());
        // let surface = unsafe { instance.create_surface(window) };
        // let adapter = instance
        //     .request_adapter(&wgpu::RequestAdapterOptions {
        //         power_preference: wgpu::PowerPreference::default(),
        //         compatible_surface: Some(&surface),
        //         force_fallback_adapter: false,
        //     })
        //     .await
        //     .unwrap();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
