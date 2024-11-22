use antiq::{
    core::{Application, Window, WindowSettings},
    widget::Rectangle,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut app = Application::new()?;

    app.run(|ctx| {
        let mut settings = WindowSettings::new();
        settings.set_title("Rectangle");

        let mut window = Window::new(ctx, settings);
        window.add_widget(Box::new(Rectangle::new()));
    });

    Ok(())
}
