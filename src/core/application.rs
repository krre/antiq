use super::{
    EventLoop, Result,
    error::ApplicationError,
    event::{EventHandler, WindowAction, WindowEvent},
    window::{Window, WindowId},
    window_manager::WindowManager,
};
use crate::{platform, renderer::Renderer};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::OnceLock,
};

static APP_LOCK: OnceLock<()> = OnceLock::new();

pub struct Application {
    name: String,
    organization: String,
    event_loop: Rc<EventLoop>,
    window_manager: Rc<RefCell<WindowManager>>,
    renderer: Rc<Renderer>,
    pub(crate) platform_application: Rc<dyn platform::PlatformApplication>,
    quit_on_last_window_closed: bool,
}

pub struct ApplicationBuilder {
    name: String,
    organization: String,
    quit_on_last_window_closed: bool,
}

struct ApplicationEventHandler {
    event_loop: Weak<EventLoop>,
    window_manager: Weak<RefCell<WindowManager>>,
    quit_on_last_window_closed: bool,
}

impl Application {
    pub fn new() -> std::result::Result<Self, ApplicationError> {
        let builder = ApplicationBuilder::new();
        builder.build()
    }

    pub fn organization(&self) -> &str {
        &self.organization
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }

    pub(crate) fn window_manager(&self) -> Weak<RefCell<WindowManager>> {
        Rc::downgrade(&self.window_manager)
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub fn event_loop(&self) -> &EventLoop {
        &self.event_loop
    }

    pub fn run(&self) -> crate::core::Result<()> {
        self.event_loop.run(Box::new(ApplicationEventHandler {
            window_manager: self.window_manager().clone(),
            event_loop: Rc::downgrade(&self.event_loop),
            quit_on_last_window_closed: self.quit_on_last_window_closed,
        }))
    }

    pub fn new_window(&self) -> Result<WindowId> {
        let window = Window::new(self)?;
        Ok(window.upgrade().map(|w| w.borrow().id()).unwrap())
    }
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self {
            name: Application::file_name().unwrap_or("Application".to_string()),
            organization: Application::file_name().unwrap_or("Antiq".to_string()),
            quit_on_last_window_closed: true,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn organization(mut self, organization: &str) -> Self {
        self.organization = organization.to_owned();
        self
    }

    pub fn quit_on_last_window_closed(mut self, quit: bool) -> Self {
        self.quit_on_last_window_closed = quit;
        self
    }

    pub fn build(self) -> std::result::Result<Application, ApplicationError> {
        if let Err(_) = APP_LOCK.set(()) {
            return Err(ApplicationError::AlreadyExists);
        }

        let mut application = Application {
            name: self.name,
            organization: self.organization,
            event_loop: Rc::new(EventLoop::new_uninit()),
            window_manager: Rc::new(RefCell::new(WindowManager::new())),
            renderer: Rc::new(Renderer::new()),
            platform_application: platform::Application::new()
                .map_err(|e| ApplicationError::Other(e))?,
            quit_on_last_window_closed: self.quit_on_last_window_closed,
        };

        let event_loop = EventLoop::new(&application).map_err(|e| ApplicationError::Other(e))?;
        application.event_loop = Rc::new(event_loop);

        Ok(application)
    }
}

impl EventHandler for ApplicationEventHandler {
    fn window_event(&self, event: WindowEvent) {
        if let Some(wm) = self.window_manager.upgrade() {
            match event.action {
                WindowAction::Redraw => {
                    wm.borrow().render(event.id);
                }
                WindowAction::Close => {
                    wm.borrow_mut().remove(event.id);

                    if wm.borrow().count() == 0 && self.quit_on_last_window_closed {
                        self.event_loop.upgrade().map(|el| el.quit());
                    }
                }
                WindowAction::Resize(size) => {
                    wm.borrow().resize(event.id, size);
                }
                WindowAction::AskResize(size) => {
                    wm.borrow().ask_resize(event.id, size);
                }
                WindowAction::Move(pos) => {
                    wm.borrow().move_to(event.id, pos);
                }
            }
        }
    }
}
