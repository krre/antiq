use antiq::{application::Application, core::Result, window::Window};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let mut window = Window::new(&app)?;
    window.set_title("Simple Window Example");

    app.run()?;

    Ok(())
}
