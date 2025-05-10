use super::Layout;

pub struct Split {}

impl Split {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Split {
    fn build(&self) {}
}
