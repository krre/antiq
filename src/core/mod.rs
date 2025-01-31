pub mod application;
pub mod event;
pub mod preferences;
pub mod window;

mod color;
mod error;
mod event_loop;
mod id;
mod layout;
mod node;
mod window_manager;

pub use color::Color;
pub use event_loop::EventLoop;
pub use id::Id;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Size2D {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Pos2D {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Border2D {
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
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

impl Border2D {
    pub fn new(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn top(&self) -> u16 {
        self.top
    }

    pub fn bottom(&self) -> u16 {
        self.bottom
    }

    pub fn left(&self) -> u16 {
        self.left
    }

    pub fn right(&self) -> u16 {
        self.right
    }

    pub fn border(&self) -> (u16, u16, u16, u16) {
        (self.left, self.right, self.top, self.bottom)
    }
}
