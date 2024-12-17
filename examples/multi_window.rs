use antiq::core::{Application, Color, Pos2D, Size2D, Window, WindowSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;

    let window_1 = Window::new(app.context().clone(), WindowSettings::new())?;
    window_1.set_title("Multi Window 1");
    window_1.set_visible(true);

    let mut settings_2 = WindowSettings::new();
    settings_2.set_color(Color::new(1.0, 0.0, 0.0, 1.0));

    let window_2 = Window::new(app.context(), settings_2)?;
    window_2.set_title("Multi Window 2");
    window_2.set_position(Pos2D::new(500, 200));
    window_2.set_size(Size2D::new(300, 300));
    window_2.set_visible(true);

    app.run();

    Ok(())
}
