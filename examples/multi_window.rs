use antiq::core::{window::Settings, Application, Color, Position, Size};

fn main() {
    env_logger::init();

    let mut app = Application::new();

    {
        let mut settings = Settings::new();
        settings.set_title("Window 1");

        app.create_window(settings);
    }

    {
        let mut settings = Settings::new();
        settings.set_title("Window 2");
        settings.set_size(Size::new(600, 400));
        settings.set_position(Position::new(500, 200));
        settings.set_color(Color::new(1.0, 0.0, 0.0, 1.0));

        app.create_window(settings);
    }

    app.run();
}
