use antiq::core::{window::WindowSettings, Application, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut app = Application::new()?;

    app.run(|ctx| {
        let mut settings = WindowSettings::new();
        settings.set_title("Window");
        settings.set_visible(true);

        let window = Window::new(ctx, settings);
    });

    Ok(())
}
