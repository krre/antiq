mod application;
mod color;
mod context;
mod error;
mod event_loop;
mod id;
mod layout;
mod node;
mod window;

pub use application::{Application, ApplicationBuilder};
pub use color::Color;
pub use context::Context;
pub use event_loop::EventLoop;
pub use id::Id;
pub use window::Window;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug)]
pub struct Size2D {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug)]
pub struct Pos2D {
    x: i32,
    y: i32,
}

impl Size2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Pos2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
