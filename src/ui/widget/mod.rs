pub mod rectangle;
pub use rectangle::Rectangle;

pub trait Widget {
    fn build(&self);
}

pub struct EmptyWidget {}

impl EmptyWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for EmptyWidget {
    fn build(&self) {}
}
