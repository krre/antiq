use super::node::Updatable;

#[derive(Default)]
pub struct Application {}

impl Updatable for Application {
    fn update(&mut self) {}
}

impl Application {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub fn name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}
