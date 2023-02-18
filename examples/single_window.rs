use antiq::core::Application;

fn main() {
    env_logger::init();

    let mut app = Application::new();

    let window = app.create_window();
    window.set_title("Window");

    app.run();
}
