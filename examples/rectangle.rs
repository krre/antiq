use antiq::{
    core::{layout, window::Settings, Application},
    widget,
};

fn main() {
    env_logger::init();

    let mut app = Application::new();

    let mut layout = layout::Box::new();
    layout.set_widget(Box::new(widget::Rectangle::new()));

    {
        let mut settings = Settings::new();
        settings.set_title("Rectangle");

        app.create_window(settings, Box::new(layout));
    }

    app.run();
}
