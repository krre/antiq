use antiq::core::{Application, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multi-window test");

    let window_1 = Window::new();
    window_1.set_title("Window 1");
    window_1.set_visible(true);

    let window_2 = Window::new();
    window_2.set_title("Window 2");
    window_2.set_visible(true);

    Application::run();

    Ok(())
}
