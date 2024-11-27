use antiq::core::{Application, Window, WindowSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;

    let _ = Window::new(app.context().clone(), WindowSettings::new());

    app.run();

    Ok(())
}
