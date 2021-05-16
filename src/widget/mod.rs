use crate::entity;

mod application_window;
mod window;

pub use application_window::ApplicationWindow;
pub use window::Window;

pub trait Widget: entity::Entity {
    fn draw(&self);
}
