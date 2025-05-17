use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Border2D {
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
}

impl Border2D {
    pub fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn set_top(&mut self, top: u16) {
        self.top = top;
    }

    pub fn top(&self) -> u16 {
        self.top
    }

    pub fn set_bottom(&mut self, bottom: u16) {
        self.bottom = bottom;
    }

    pub fn bottom(&self) -> u16 {
        self.bottom
    }

    pub fn set_left(&mut self, left: u16) {
        self.left = left;
    }

    pub fn left(&self) -> u16 {
        self.left
    }

    pub fn set_right(&mut self, right: u16) {
        self.right = right;
    }

    pub fn right(&self) -> u16 {
        self.right
    }

    pub fn as_tuple(&self) -> (u16, u16, u16, u16) {
        (self.left, self.right, self.top, self.bottom)
    }
}
