use antiq::{
    application::Application,
    core::{Result, UpgradeOrErr},
    window::Window,
};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;
    let window = Window::new(&app)?;

    {
        let w = window.upgrade_or_err()?;
        let mut w = w.borrow_mut();
        w.set_title("Simple Window Example");
    }

    app.run()?;

    Ok(())
}
