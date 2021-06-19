use crate::core::Id;
use crate::entity::Entity;

#[derive(Default)]
pub struct Application {
    id: Id,
}

impl Application {
    pub fn new() -> Self {
        Self { id: Id::default() }
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
    fn id(&self) -> Id {
        self.id
    }
}
