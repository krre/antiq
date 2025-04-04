use std::rc::Rc;

use crate::core::Result;
use crate::platform::PlatformApplication;
use x11rb::xcb_ffi::XCBConnection;

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        _NET_FRAME_EXTENTS,
        UTF8_STRING,

        QUIT_EVENT,
        CLIENT_EVENT,
    }
}

pub struct Application {
    pub(crate) connection: Rc<XCBConnection>,
    pub(crate) screen_num: usize,
    pub(crate) atoms: Atoms,
}

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>> {
        let (conn, screen_num) = XCBConnection::connect(None)?;
        let atoms = Atoms::new(&conn)?.reply()?;

        Ok(Rc::new(Self {
            connection: Rc::new(conn),
            screen_num,
            atoms,
        }))
    }
}

impl PlatformApplication for Application {}
