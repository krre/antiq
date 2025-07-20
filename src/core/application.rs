pub trait Application: Default {
    fn run(&self);
}

pub struct ApplicationBackend<App: Application> {
    app: App,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new(app: App) -> Self {
        Self { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}
