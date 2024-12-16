use antiq::core::{Application, Window, WindowSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::new(app.context().clone(), WindowSettings::new())?;
    window.set_title("Simple Window");

    app.run();

    Ok(())
}
