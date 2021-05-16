use crate::core::Id;

pub mod application;

pub trait Entity {
    fn id(&self) -> Id;
}
