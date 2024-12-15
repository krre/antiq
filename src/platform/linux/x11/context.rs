use std::{any::Any, rc::Rc};

use x11rb::rust_connection::RustConnection;

use crate::platform::{PlatformApplication, PlatformContext};

use super::Application;

pub struct Context {
    connection: Rc<RustConnection>,
}

impl Context {
    pub fn new(
        app: &dyn PlatformApplication,
    ) -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        let x11_app = app.as_any().downcast_ref::<Application>().unwrap();

        Ok(Box::new(Self {
            connection: x11_app.connection().clone(),
        }))
    }

    pub fn connection(&self) -> Rc<RustConnection> {
        self.connection.clone()
    }
}

impl PlatformContext for Context {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
