use std::rc::Rc;
use wayland_client::Connection;

use crate::platform::PlatformApplication;

pub struct Application {
    connection: Rc<Connection>,
}

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let connection = Rc::new(Connection::connect_to_env()?);
        Ok(Box::new(Self { connection }))
    }

    pub fn connection(&self) -> Rc<Connection> {
        self.connection.clone()
    }
}

impl PlatformApplication for Application {}
