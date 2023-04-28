use antiq::core::{layout, Application, Color, Position, Size};

fn main() {
    env_logger::init();

    let mut app = Application::new();

    let window_id_1 = app.create_window(Box::new(layout::Box::new()));
    app.window_mut(window_id_1).set_title("Window 1");

    let window_id_2 = app.create_window(Box::new(layout::Box::new()));

    {
        let mut window_2 = app.window_mut(window_id_2);
        window_2.set_size(Size::new(600, 400));
        window_2.set_position(Position::new(500, 200));
        window_2.set_color(Color::new(1.0, 0.0, 0.0, 1.0));
    }

    app.run();
}
