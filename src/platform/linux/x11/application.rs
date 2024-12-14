use std::rc::Rc;

use crate::platform::PlatformApplication;
use x11rb::rust_connection::RustConnection;

pub struct Application {
    connection: Rc<RustConnection>,
    screen_num: usize,
}

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let (conn, screen_num) = x11rb::connect(None)?;

        Ok(Box::new(Self {
            connection: Rc::new(conn),
            screen_num,
        }))
    }

    pub fn connection(&self) -> Rc<RustConnection> {
        self.connection.clone()
    }

    pub fn screen_num(&self) -> usize {
        self.screen_num
    }
}

impl PlatformApplication for Application {}