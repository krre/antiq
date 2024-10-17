use antiq::{
    core::{window::Settings, Application},
    widget::Rectangle,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut app = Application::new()?;

    {
        let mut settings = Settings::new();
        settings.set_title("Rectangle");

        let mut window = app.create_window(settings);
        window.add_widget(Box::new(Rectangle::new()));
    }

    app.run();

    Ok(())
}
