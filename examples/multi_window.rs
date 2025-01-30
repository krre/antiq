use antiq::{
    application::Application,
    core::{Color, Pos2D, Size2D},
    window::Window,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;
    let window_1 = Window::new(&app)?;

    {
        let w = window_1.upgrade().unwrap();
        w.set_title("Multi Window 1");
    }

    let window_2 = Window::new(&app)?;

    {
        let w = window_2.upgrade().unwrap();
        w.set_title("Multi Window 2");
        w.set_position(Pos2D::new(500, 200));
        w.set_size(Size2D::new(300, 300));
        w.set_color(Color::new(1.0, 0.0, 0.0));
    }

    app.run()?;

    Ok(())
}
