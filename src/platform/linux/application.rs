use std::rc::Rc;

use crate::platform::PlatformApplication;

pub struct Application {
    connection: Rc<wayland_client::Connection>,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Rc::new(wayland_client::Connection::connect_to_env()?);

        Ok(Self { connection })
    }
}

impl PlatformApplication for Application {}
