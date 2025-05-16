use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Column {}

impl Column {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Column {
    fn build(&self) {}
}

impl Layout2D for Column {}
