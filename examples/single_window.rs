use antiq::core::Application;

fn main() {
    let app = Application::new();

    let window = app.create_window();
    window.set_title("Window");

    app.run();
}
