use super::{
    EventLoop,
    error::ApplicationError,
    event::{EventHandler, WindowAction, WindowEvent},
    window_manager::WindowManager,
};
use crate::{platform, renderer::Renderer};
use std::{rc::Rc, sync::OnceLock};

static APP_LOCK: OnceLock<()> = OnceLock::new();

pub struct Application {
    name: String,
    organization: String,
    event_loop: EventLoop,
    window_manager: Rc<WindowManager>,
    renderer: Rc<Renderer>,
    pub(crate) platform_application: Rc<dyn platform::PlatformApplication>,
    quit_on_last_window_closed: bool,
}

pub struct ApplicationBuilder {
    name: String,
    organization: String,
    quit_on_last_window_closed: bool,
}

struct ApplicationEventHandler<'app>(&'app Application);

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
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

    pub(crate) fn window_manager(&self) -> Rc<WindowManager> {
        self.window_manager.clone()
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub fn event_loop(&self) -> &EventLoop {
        &self.event_loop
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let event_handler = ApplicationEventHandler { 0: self };
        self.event_loop.run(&event_handler)
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

    pub fn build(self) -> Result<Application, ApplicationError> {
        if let Err(_) = APP_LOCK.set(()) {
            return Err(ApplicationError::AlreadyExists);
        }

        let platform_application =
            platform::Application::new().map_err(|e| ApplicationError::Other(e))?;

        let mut application = Application {
            name: self.name,
            organization: self.organization,
            event_loop: EventLoop::new_uninit(),
            window_manager: Rc::new(WindowManager::new()),
            renderer: Rc::new(Renderer::new()),
            platform_application: platform_application.clone(),
            quit_on_last_window_closed: self.quit_on_last_window_closed,
        };

        let event_loop = EventLoop::new(&application).map_err(|e| ApplicationError::Other(e))?;
        let platform_event_loop = event_loop.platform_event_loop.clone();
        platform_application.init(platform_event_loop);

        application.event_loop = event_loop;

        Ok(application)
    }
}

impl<'app> EventHandler for ApplicationEventHandler<'app> {
    fn window_event(&self, event: WindowEvent) {
        match event.action {
            WindowAction::Redraw => {
                self.0.window_manager().render(event.id);
            }
            WindowAction::Close => {
                self.0.window_manager().remove(event.id);

                if self.0.window_manager().count() == 0 && self.0.quit_on_last_window_closed {
                    self.0.event_loop.quit();
                }
            }
            WindowAction::Resize(size) => {
                self.0.window_manager().resize(event.id, size);
            }
            WindowAction::Move(pos) => {
                self.0.window_manager().move_to(event.id, pos);
            }
        }
    }
}
