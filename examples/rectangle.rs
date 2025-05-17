use antiq::{
    application::Application,
    core::{Result, UpgradeOrErr},
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
    let window = Window::new(&app)?;

    {
        let w = window.upgrade_or_err()?;
        let mut w = w.borrow_mut();
        w.set_title("Rectangle Example");
        w.set_visible(true);

        let mut rect = Rectangle::new();
        rect.set_size(Size2D::new(200, 100));

        let mut row = Row2D::new();
        row.add_widget(rect);

        w.set_layout(Box::new(row));
    }

    app.run()?;

    Ok(())
}
