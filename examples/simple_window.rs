use antiq::core::{Application, Window, WindowSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut app = Application::new()?;

    app.run(|ctx| {
        let mut settings = WindowSettings::new();
        settings.set_title("Window");

        let window = Window::new(ctx, settings);
    });

    Ok(())
}
