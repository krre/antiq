use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Size2D {
    width: u32,
    height: u32,
}

impl Size2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn as_tuple(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
