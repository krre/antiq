use antiq::{
    application::Application,
    core::{Result, UpgradeOrErr},
    ui::{
        Color,
        d2::geometry::{Pos2D, Size2D},
    },
    window::Window,
};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let window_1 = Window::new(&app)?;

    {
        let w = window_1.upgrade_or_err()?;
        w.borrow_mut().set_title("Multi Window 1 Example");
    }

    let window_2 = Window::new(&app)?;

    {
        let w = window_2.upgrade_or_err()?;
        let mut w = w.borrow_mut();
        w.set_title("Multi Window 2 Example");
        w.set_position(Pos2D::new(500, 200));
        w.set_size(Size2D::new(300, 300));
        w.set_color(Color::new(1.0, 0.0, 0.0));
    }

    app.run()?;

    Ok(())
}
