mod application;

pub use application::Application;

use crate::core::EntityCore;

pub trait Entity {
    fn entity_ref(&self) -> &EntityCore;
    fn entity_mut(&mut self) -> &EntityCore;
}
