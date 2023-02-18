use antiq::core::{Application, Color};

fn main() {
    let mut app = Application::new();

    let window = app.create_window();
    window.set_title("Dot");
    window.set_color(Color::new(0.2, 0.2, 0.2, 1.0));

    app.run();
}
