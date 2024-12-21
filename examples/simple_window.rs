use antiq::core::{Application, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;

    let window = Window::new(app.context().clone())?;
    window.set_title("Simple Window");
    window.set_visible(true);
    window.render();

    app.run()?;

    Ok(())
}
