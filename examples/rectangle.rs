use antiq::{
    application::Application,
    core::Result,
    ui::d2::{
        geometry::Size2D,
        layout::Row2D,
        widget::{Rectangle, Widget2D},
    },
    window::Window,
};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let mut window = Window::new(&app)?;
    window.set_title("Rectangle Example");

    let mut rect = Rectangle::new();
    rect.set_size(Size2D::new(200, 100));

    let mut row = Row2D::new();
    row.add_widget2d(rect);

    window.set_layout(Box::new(row));

    app.run()?;

    Ok(())
}
