use antiq::entity::Application;
use antiq::widget::{ApplicationWindow, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multi-window test");

    let mut app = Application::new();

    let mut app_window = ApplicationWindow::new(&mut app)?;
    app_window.set_title("Applicatin Window");
    app_window.set_visible(true);
    app.add_window(app_window);

    let mut window = Window::new(&mut app)?;
    window.set_title("Window");
    window.set_visible(true);
    app.add_window(window);

    app.run()?;

    Ok(())
}
