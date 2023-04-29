use antiq::core::{layout, Application};

fn main() {
    env_logger::init();

    let mut app = Application::new();

    {
        let mut window = app.create_window(Box::new(layout::Box::new()));
        window.set_title("Window");
    }

    app.run();
}
