use antiq::core::{Application, Window};

fn main() {
    println!("Multi-window test");

    let window_1 = Window::new();
    window_1.set_title("Window 1");
    window_1.show();

    let window_2 = Window::new();
    window_2.set_title("Window 2");
    window_2.set_size(600, 400);
    window_2.set_position(200, 200);
    window_2.show();

    Application::run();
}
