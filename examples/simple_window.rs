use antiq::{application::Application, window::Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;
    let window = Window::new(&app)?;

    let w = window.upgrade().unwrap();
    w.set_title("Simple Window");
    w.set_visible(true);
    drop(w);

    app.run()?;

    Ok(())
}
