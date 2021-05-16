use crate::core::Id;

mod application;

pub use application::Application;

pub trait Entity {
    fn id(&self) -> Id;
}
