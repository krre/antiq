use antiq::core::{layout, Application, Color};

fn main() {
    env_logger::init();

    let mut app = Application::new();

    let window = app.create_window(Box::new(layout::Box::new()));
    window.set_title("Dot");
    window.set_color(Color::new(0.2, 0.2, 0.2, 1.0));

    app.run();
}
