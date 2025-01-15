use std::{any::Any, rc::Rc};

use x11rb::xcb_ffi::XCBConnection;

use crate::platform::{PlatformApplication, PlatformContext};

use super::Application;

pub struct Context {
    pub(crate) connection: Rc<XCBConnection>,
    pub(crate) screen_num: usize,
    pub(crate) atoms: Atoms,
}

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        _NET_FRAME_EXTENTS,
        UTF8_STRING,
    }
}

impl Context {
    pub fn new(
        app: &dyn PlatformApplication,
    ) -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        let x11_app = app.as_any().downcast_ref::<Application>().unwrap();
        let atoms = Atoms::new(&x11_app.connection)?.reply()?;

        Ok(Box::new(Self {
            connection: x11_app.connection.clone(),
            screen_num: x11_app.screen_num,
            atoms,
        }))
    }
}

impl PlatformContext for Context {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
