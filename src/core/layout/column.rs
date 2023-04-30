use super::Layout;

pub struct Column {}

impl Column {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Column {
    fn build(&self) {}
}
