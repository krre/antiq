mod application_window;
mod window;

use crate::core::{EntityCore, WidgetCore};
pub use crate::platform::*;
pub use application_window::ApplicationWindow;
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
