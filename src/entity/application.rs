use crate::core::EntityCore;
use crate::entity::Entity;
use crate::widget::WindowWidget;
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Debug)]
pub struct Application {
    entity_core: EntityCore,
    windows: Vec<Box<dyn WindowWidget>>,
    event_loop: EventLoop<()>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            entity_core: EntityCore::default(),
            windows: Vec::new(),
            event_loop: EventLoop::new(),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            println!("{:?}", event);
            match event {
                _ => (),
            }
        });

        Ok(())
    }

    pub fn name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }

    pub fn add_window<W: WindowWidget + 'static>(&mut self, window: W) {
        self.windows.push(Box::new(window));
    }

    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }
}

impl Entity for Application {
    fn entity_ref(&self) -> &EntityCore {
        &self.entity_core
    }

    fn entity_mut(&mut self) -> &mut EntityCore {
        &mut self.entity_core
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
