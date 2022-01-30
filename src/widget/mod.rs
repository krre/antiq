mod application_window;
mod window;

use crate::core::{EntityCore, WidgetCore};
pub use application_window::ApplicationWindow;
use core::fmt::Debug;
pub use window::Window;

pub trait Widget {
    fn widget_ref(&self) -> &WidgetCore;

    fn widget_mut(&mut self) -> &mut WidgetCore;

    fn entity_ref(&self) -> &EntityCore {
        self.widget_ref().entity_ref()
    }

    fn entity_mut(&mut self) -> &mut EntityCore {
        self.widget_mut().entity_mut()
    }

    fn draw(&self);
}

pub trait WindowWidget {
    fn id(&self) -> winit::window::WindowId;

    fn render(&self);
}

impl Debug for dyn WindowWidget {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "WindowWidget")
    }
}
