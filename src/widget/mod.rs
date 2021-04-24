use crate::core::{self, node};
use crate::window;

pub type Application = node::UpdatedNode<core::application::Application>;
pub type Window = node::UpdatedNode<window::Window>;
pub type ApplicationWindow = node::UpdatedNode<window::ApplicationWindow>;
