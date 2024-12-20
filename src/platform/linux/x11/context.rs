use std::{any::Any, rc::Rc};

use x11rb::xcb_ffi::XCBConnection;

use crate::platform::{PlatformApplication, PlatformContext};

use super::Application;

pub struct Context {
    pub(crate) connection: Rc<XCBConnection>,
    pub(crate) screen_num: usize,
}

impl Context {
    pub fn new(
        app: &dyn PlatformApplication,
    ) -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        let x11_app = app.as_any().downcast_ref::<Application>().unwrap();

        Ok(Box::new(Self {
            connection: x11_app.connection.clone(),
            screen_num: x11_app.screen_num,
        }))
    }
}

impl PlatformContext for Context {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
