use antiq::core::{Application, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multi-window test");

    let window_1 = Window::new()?.upgrade().unwrap();
    window_1.borrow_mut().set_title("Window 1");
    window_1.borrow_mut().set_visible(true);

    let window_2 = Window::new()?.upgrade().unwrap();
    window_2.borrow_mut().set_title("Window 2");
    window_2.borrow_mut().set_visible(true);

    drop(window_1);
    drop(window_2);

    Application::run();

    Ok(())
}
