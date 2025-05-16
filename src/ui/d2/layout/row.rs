use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Row {}

impl Row {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Row {
    fn build(&self) {}
}

impl Layout2D for Row {}
