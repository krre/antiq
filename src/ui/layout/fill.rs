use super::Layout;

pub struct Fill {}

impl Fill {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Fill {
    fn build(&self) {}
}
