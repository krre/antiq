use antiq::{
    application::Application,
    core::Result,
    ui::{
        Color,
        d2::geometry::{Pos2D, Size2D},
    },
    window::Window,
};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let mut window_1 = Window::new(&app)?;
    window_1.set_title("Multi Window 1 Example");

    let mut window_2 = Window::new(&app)?;
    window_2.set_title("Multi Window 2 Example");
    window_2.set_position(Pos2D::new(500, 200));
    window_2.set_size(Size2D::new(300, 300));
    window_2.set_color(Color::new(1.0, 0.0, 0.0));

    app.run()?;

    Ok(())
}
