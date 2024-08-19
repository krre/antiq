use super::Layout;

pub struct Linear {}

impl Linear {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Linear {
    fn build(&self) {
        log::info!("Build linear layout");
    }
}
