use super::Layout2D;

pub struct Split {}

impl Split {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout2D for Split {
    fn build(&self) {}
}
