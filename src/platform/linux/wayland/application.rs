use std::rc::Rc;

use crate::platform::PlatformApplication;

pub struct Application {
    connection: Rc<wayland_client::Connection>,
}

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let connection = Rc::new(wayland_client::Connection::connect_to_env()?);

        Ok(Box::new(Self { connection }))
    }
}

impl PlatformApplication for Application {}
