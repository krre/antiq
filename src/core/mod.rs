pub mod layout;
pub mod node;
pub mod window;

mod application;
mod color;
mod id;

pub use application::Application;
pub use color::Color;
pub use id::Id;
use serde::{Deserialize, Serialize};
pub use window::Window;

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Size {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}
