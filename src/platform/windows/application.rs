use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::*;

use std::rc::Rc;

use crate::core::Result;
use crate::platform::PlatformApplication;

pub struct Application {
    pub(crate) hinstance: HMODULE,
}

impl Application {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hinstance: unsafe { GetModuleHandleW(None) }?,
        })
    }
}

impl PlatformApplication for Application {}

pub fn new_application() -> Result<Rc<dyn PlatformApplication>> {
    Ok(Rc::new(Application::new()?))
}
