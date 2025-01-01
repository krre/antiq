use std::{any::Any, rc::Rc};

use wayland_client::Connection;

use crate::platform::{PlatformApplication, PlatformContext};

use super::Application;

pub struct Context {
    connection: Rc<Connection>,
}

impl Context {
    pub fn new(
        app: &dyn PlatformApplication,
    ) -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        let wayland_app = app.as_any().downcast_ref::<Application>().unwrap();

        Ok(Box::new(Self {
            connection: wayland_app.connection.clone(),
        }))
    }
}

impl PlatformContext for Context {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
