use antiq::core::{Application, Color, Pos2D, Size2D, Window, WindowSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;

    let mut settings1 = WindowSettings::new();
    settings1.set_title("Window 1");

    let _ = Window::new(app.context().clone(), settings1);

    let mut settings2 = WindowSettings::new();
    settings2.set_title("Window 2");
    settings2.set_size(Size2D::new(600, 400));
    settings2.set_position(Pos2D::new(500, 200));
    settings2.set_color(Color::new(1.0, 0.0, 0.0, 1.0));

    let _ = Window::new(app.context(), settings2);

    app.run();

    Ok(())
}
