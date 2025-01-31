use std::{any::Any, rc::Rc};
use wayland_client::Connection;

use crate::platform::PlatformApplication;

pub struct Application {
    pub(crate) connection: Rc<Connection>,
}

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let connection = Rc::new(Connection::connect_to_env()?);
        Ok(Rc::new(Self { connection }))
    }

    pub fn connection(&self) -> Rc<Connection> {
        self.connection.clone()
    }
}

impl PlatformApplication for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
