use antiq::core::{Application, Color, Pos2D, Size2D, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;

    let window_1 = Window::new(app.context().clone())?;
    window_1.set_title("Multi Window 1");
    window_1.set_visible(true);

    let window_2 = Window::new(app.context())?;
    window_2.set_title("Multi Window 2");
    window_2.set_visible(true);
    window_2.set_position(Pos2D::new(500, 200));
    window_2.set_size(Size2D::new(300, 300));
    window_2.set_color(Color::new(1.0, 0.0, 0.0, 1.0));

    app.run();

    Ok(())
}
