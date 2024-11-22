use antiq::core::{Application, Color, Position, Size, Window, WindowSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut app = Application::new()?;

    app.run(|ctx| {
        {
            let mut settings = WindowSettings::new();
            settings.set_title("Window 1");

            let _ = Window::new(ctx.clone(), settings);
        }

        {
            let mut settings = WindowSettings::new();
            settings.set_title("Window 2");
            settings.set_size(Size::new(600, 400));
            settings.set_position(Position::new(500, 200));
            settings.set_color(Color::new(1.0, 0.0, 0.0, 1.0));

            let _ = Window::new(ctx, settings);
        }
    });

    Ok(())
}
