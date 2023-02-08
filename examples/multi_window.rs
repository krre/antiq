use antiq::core::Application;

fn main() {
    let mut app = Application::new();

    let window_1 = app.create_window();
    window_1.set_title("Window 1");

    let window_2 = app.create_window();
    window_2.set_title("Window 2");
    window_2.set_size(600, 400);
    window_2.set_position(500, 200);

    app.run();
}
