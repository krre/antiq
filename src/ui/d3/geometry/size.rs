use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Size3D {
    width: u32,
    height: u32,
    depth: u32,
}

impl Size3D {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
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

    pub fn set_depth(&mut self, depth: u32) {
        self.depth = depth;
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn as_tuple(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.depth)
    }
}
