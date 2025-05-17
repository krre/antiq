use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Split2D {}

impl Split2D {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Split2D {
    fn build(&self) {}
}

impl Layout2D for Split2D {}
