pub mod layout;
pub mod node;
pub mod window;

mod application;
mod color;
mod id;

pub use application::Application;
pub use color::Color;
pub use id::Id;
pub use window::Window;

pub struct Size {
    width: u32,
    height: u32,
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
