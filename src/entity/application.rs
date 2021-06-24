use crate::core::EntityCore;
use crate::entity::Entity;
use crate::platform::PlatformApplication;

#[derive(Default, Debug)]
pub struct Application {
    entity_core: EntityCore,
    platform_application: PlatformApplication,
}

impl Application {
    pub fn new() -> Self {
        Self {
            entity_core: EntityCore::default(),
            platform_application: PlatformApplication::default(),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
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
}

impl Entity for Application {
    fn entity_ref(&self) -> &EntityCore {
        &self.entity_core
    }

    fn entity_mut(&mut self) -> &mut EntityCore {
        &mut self.entity_core
    }
}
