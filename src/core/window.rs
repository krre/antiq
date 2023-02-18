use std::cell::RefCell;

use crate::gfx::Gpu;
use winit::{self, dpi::PhysicalPosition};

use super::{Application, Color};

#[derive(Debug, Clone, Copy)]
pub struct Id(winit::window::WindowId);

#[derive(Debug)]
pub struct Window {
    id: Id,
    title: RefCell<String>,
    winit_window: winit::window::Window,
    wgpu_surface: wgpu::Surface,
    wgpu_config: RefCell<wgpu::SurfaceConfiguration>,
    color: RefCell<Color>,
    position: RefCell<PhysicalPosition<i32>>,
}

impl Id {
    pub(crate) fn new(winit_id: winit::window::WindowId) -> Self {
        Self(winit_id)
    }

    pub(crate) fn winit_id(&self) -> winit::window::WindowId {
        self.0
    }
}

impl Window {
    pub(crate) fn new(app: &Application) -> Self {
        let winit_window = winit::window::WindowBuilder::new()
            .build(&app.event_loop())
            .unwrap();

        let id = Id::new(winit_window.id());

        let wgpu_surface = unsafe {
            app.engine()
                .gpu()
                .instance()
                .create_surface(&winit_window)
                .unwrap()
        };

        let caps = wgpu_surface.get_capabilities(app.engine().gpu().adapter());
        let size = winit_window.inner_size();

        let wgpu_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        wgpu_surface.configure(app.engine().gpu().device(), &wgpu_config);

        Self {
            id,
            title: RefCell::new(String::from("")),
            winit_window,
            wgpu_surface,
            wgpu_config: RefCell::new(wgpu_config),
            color: RefCell::new(Color::new(0.0, 0.0, 1.0, 1.0)),
            position: RefCell::new(PhysicalPosition::default()),
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn set_title(&self, title: &str) {
        self.winit_window.set_title(title);
        *self.title.borrow_mut() = String::from(title);
    }

    pub fn set_visible(&self, visible: bool) {
        self.winit_window.set_visible(visible);
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.winit_window
            .set_inner_size(winit::dpi::PhysicalSize::new(width, height));
    }

    pub fn size(&self) -> (u32, u32) {
        (
            self.winit_window.inner_size().width,
            self.winit_window.inner_size().height,
        )
    }

    pub fn set_position(&self, x: i32, y: i32) {
        let pos = winit::dpi::PhysicalPosition::new(x, y);
        self.winit_window.set_outer_position(pos);
        self.set_cache_position(pos)
    }

    pub(crate) fn set_cache_position(&self, position: PhysicalPosition<i32>) {
        *self.position.borrow_mut() = position;
    }

    pub fn position(&self) -> (i32, i32) {
        let pos = self.position.borrow();
        (pos.x, pos.y)
    }

    pub fn set_color(&self, color: Color) {
        *self.color.borrow_mut() = color
    }

    pub fn set_maximized(&self, maximized: bool) {
        self.winit_window.set_maximized(maximized);
    }

    pub fn is_maximized(&self) -> bool {
        self.winit_window.is_maximized()
    }

    pub fn resize(&self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.wgpu_config.borrow_mut().width = size.width;
        self.wgpu_config.borrow_mut().height = size.height;
        self.wgpu_surface
            .configure(device, &self.wgpu_config.borrow());
        self.winit_window.request_redraw();
    }

    pub fn draw(&self) {
        log::info!("Draw window: {}", self.winit_window.title());
    }

    pub fn render(&self, gpu: &Gpu) {
        log::info!("Render window: {}", self.title.borrow());

        let frame = self.wgpu_surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = gpu
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color.borrow().inner()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        gpu.queue().submit(Some(encoder.finish()));
        frame.present();
    }
}
