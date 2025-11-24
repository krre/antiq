use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Pos3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
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

    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn as_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}
