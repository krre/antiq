use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

pub struct ShaderStorage {
    shaders: RefCell<HashMap<ShaderName, Rc<wgpu::ShaderModule>>>,
}

#[derive(Eq, Hash, PartialEq)]
pub enum ShaderName {
    Dot,
}

impl ShaderStorage {
    pub fn new() -> Self {
        Self {
            shaders: RefCell::new(HashMap::new()),
        }
    }

    pub fn load(&self, device: &wgpu::Device) {
        self.load_shader(device, ShaderName::Dot, include_str!("sources/dot.wgsl"));
    }

    fn load_shader(&self, device: &wgpu::Device, name: ShaderName, source: &str) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
        });

        self.shaders.borrow_mut().insert(name, Rc::new(shader));
    }
}
