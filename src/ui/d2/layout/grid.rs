use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Grid {}

impl Grid {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for Grid {
    fn build(&self) {}
}

impl Layout2D for Grid {}
