use crate::core::EntityCore;
use crate::entity::Entity;
use crate::platform::PlatformApplication;
use crate::widget::WindowWidget;

#[derive(Debug)]
pub struct Application {
    entity_core: EntityCore,
    platform_application: PlatformApplication,
    windows: Vec<Box<dyn WindowWidget>>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            entity_core: EntityCore::default(),
            platform_application: PlatformApplication::default(),
            windows: Vec::new(),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
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
