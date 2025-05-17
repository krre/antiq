use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Pos2D {
    x: i32,
    y: i32,
}

impl Pos2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
