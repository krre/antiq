use super::Layout2D;

pub struct Row {}

impl Row {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout2D for Row {
    fn build(&self) {}
}
