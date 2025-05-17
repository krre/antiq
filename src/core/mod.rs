pub mod application;
pub mod event;
pub mod preferences;
pub mod window;

mod color;
mod event_loop;
mod id;
mod window_manager;

pub use color::Color;
pub use event_loop::EventLoop;
pub use id::Id;

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Size2D {
    width: u32,
    height: u32,
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

pub trait UpgradeOrErr<T> {
    fn upgrade_or_err(self) -> std::result::Result<T, &'static str>;
}

impl<T> UpgradeOrErr<std::rc::Rc<T>> for std::rc::Weak<T> {
    fn upgrade_or_err(self) -> std::result::Result<std::rc::Rc<T>, &'static str> {
        self.upgrade().ok_or("Window weak reference is invalid")
    }
}
