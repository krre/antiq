use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Split {}

impl Split {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Split {
    fn build(&self) {}
}

impl Layout2D for Split {}
