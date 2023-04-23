use antiq::core::{layout, Application};

fn main() {
    env_logger::init();

    let mut app = Application::new();

    let window_id = app.create_window(Box::new(layout::Box::new()));
    app.window_mut(window_id).set_title("Window");

    app.run();
}
