mod application_window;
mod window;

pub use application_window::*;
pub use window::*;

pub trait Windowed {
    fn set_title(&mut self, title: &str);
    fn title(&self) -> &str;
}
