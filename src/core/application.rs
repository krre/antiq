use super::{window, Window};
use pollster;
use std::collections::HashMap;
use std::ops::Deref;
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

#[derive(Debug)]
pub struct Application {
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Box<window::Window>>,
    wgpu_instance: wgpu::Instance,
    wgpu_adapter: wgpu::Adapter,
    wgpu_device: wgpu::Device,
    wgpu_queue: wgpu::Queue,
}

impl Application {
    pub fn new() -> Self {
        let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        let adapter_options = wgpu::RequestAdapterOptions {
            ..Default::default()
        };

        let wgpu_adapter =
            pollster::block_on(wgpu_instance.request_adapter(&adapter_options)).unwrap();

        println!("Graphic adapter: {}", wgpu_adapter.get_info().name);

        let device_descriptor = wgpu::DeviceDescriptor::default();
        let device_future = wgpu_adapter.request_device(&device_descriptor, None);
        let (wgpu_device, wgpu_queue) = pollster::block_on(device_future).unwrap();

        Self {
            event_loop: EventLoop::new(),
            windows: HashMap::new(),
            wgpu_instance,
            wgpu_adapter,
            wgpu_device,
            wgpu_queue,
        }
    }

    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub(crate) fn wgpu_instance(&self) -> &wgpu::Instance {
        &&self.wgpu_instance
    }

    pub fn create_window(&mut self) -> &Window {
        let window = Box::new(Window::new(self));
        let id = window.id().winit_id();
        self.windows.insert(id, window);
        self.windows.get(&id).unwrap().deref()
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::RedrawRequested(window_id) => {
                    self.windows.get(&window_id).unwrap().render();
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    self.windows.get(&window_id).unwrap().set_visible(false);
                    self.windows.remove(&window_id);

                    if self.windows.len() == 0 {
                        control_flow.set_exit();
                    }
                }

                _ => (),
            }
        });
    }

    pub fn name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }
}
