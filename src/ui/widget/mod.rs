mod rectangle;
mod view;

pub use rectangle::Rectangle;
pub use view::View;

pub trait Widget {
    fn build(&self);
}

impl Widget for () {
    fn build(&self) {}
}
