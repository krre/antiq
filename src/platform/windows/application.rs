use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::*;

use std::any::Any;
use std::rc::Rc;

use crate::core::Result;
use crate::platform::PlatformApplication;

pub struct Application {
    pub(crate) hinstance: HMODULE
}

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>> {
        Ok(Rc::new(Self {
            hinstance: unsafe { GetModuleHandleW(None) }?
        }))
    }
}

impl PlatformApplication for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
