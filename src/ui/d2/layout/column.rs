use super::Layout2D;

pub struct Column {}

impl Column {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout2D for Column {
    fn build(&self) {}
}
