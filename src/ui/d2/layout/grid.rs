use crate::ui::layout::Layout;

use super::Layout2D;

pub struct Grid2D {}

impl Grid2D {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Grid2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout for Grid2D {
    fn build(&self) {}
}

impl Layout2D for Grid2D {}
