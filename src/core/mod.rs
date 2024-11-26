pub mod event_loop;
pub mod layout;
pub mod node;
pub mod window;

mod application;
mod color;
mod context;
mod id;

pub use application::{Application, ApplicationBuilder};
pub use color::Color;
pub use context::AppContext;
pub use event_loop::EventLoop;
pub use id::Id;
pub use window::Window;
pub use window::WindowSettings;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug)]
pub struct Size {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug)]
pub struct Pos2d {
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

impl Pos2d {
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
