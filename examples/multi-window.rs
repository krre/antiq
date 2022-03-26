use antiq::entity::Application;
use antiq::widget::Window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multi-window test");

    let mut app = Application::new();

    let mut window_1 = Window::new(&mut app)?;
    window_1.set_title("Window 1");
    window_1.set_visible(true);
    app.add_window(window_1);

    let mut window_2 = Window::new(&mut app)?;
    window_2.set_title("Window 2");
    window_2.set_visible(true);
    app.add_window(window_2);

    app.run()?;

    Ok(())
}
