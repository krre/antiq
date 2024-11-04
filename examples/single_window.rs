use antiq::core::{window::WindowSettings, Application};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut app = Application::new()?;

    {
        let mut settings = WindowSettings::new();
        settings.set_title("Window");

        app.create_window(settings);
    }

    app.run();

    Ok(())
}
