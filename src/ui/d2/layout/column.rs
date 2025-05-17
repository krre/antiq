use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Column2D {}

impl Column2D {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Column2D {
    fn build(&self) {}
}

impl Layout2D for Column2D {}
