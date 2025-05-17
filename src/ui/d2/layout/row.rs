use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Row2D {}

impl Row2D {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Row2D {
    fn build(&self) {}
}

impl Layout2D for Row2D {}
