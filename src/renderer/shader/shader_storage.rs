use std::{borrow::Cow, collections::HashMap, rc::Rc};

type Shaders = HashMap<ShaderName, Rc<wgpu::ShaderModule>>;

pub struct ShaderStorage {
    shaders: Shaders,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum ShaderName {
    Dot,
}

impl ShaderStorage {
    pub fn new(device: &wgpu::Device) -> Self {
        let shaders = HashMap::from([(
            ShaderName::Dot,
            Self::load_shader(device, include_str!("sources/dot.wgsl")),
        )]);

        Self { shaders }
    }

    pub fn shader(&self, name: ShaderName) -> &wgpu::ShaderModule {
        self.shaders
            .get(&name)
            .unwrap_or_else(|| panic!("Shader {:?} not found", name))
    }

    fn load_shader(device: &wgpu::Device, source: &str) -> Rc<wgpu::ShaderModule> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
        });

        Rc::new(shader)
    }
}
