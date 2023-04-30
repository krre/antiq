use super::Layout;

pub struct Linear {}

impl Linear {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Linear {
    fn draw(&self) {
        log::info!("Draw linear layout");
    }
}
