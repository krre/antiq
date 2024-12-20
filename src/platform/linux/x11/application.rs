use std::{any::Any, rc::Rc};

use crate::platform::PlatformApplication;
use x11rb::xcb_ffi::XCBConnection;

pub struct Application {
    pub(crate) connection: Rc<XCBConnection>,
    pub(crate) screen_num: usize,
}

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        let (conn, screen_num) = XCBConnection::connect(None)?;

        Ok(Box::new(Self {
            connection: Rc::new(conn),
            screen_num,
        }))
    }
}

impl PlatformApplication for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
