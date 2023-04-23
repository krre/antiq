use antiq::{
    core::{layout, Application},
    widget,
};

fn main() {
    env_logger::init();

    let mut app = Application::new();

    let mut layout = layout::Box::new();
    layout.set_widget(Box::new(widget::Rectangle::new()));

    let window = app.create_window(Box::new(layout));
    window.set_title("Rectangle");

    app.run();
}
