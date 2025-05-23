use antiq::{
    application::Application,
    core::{Result, UpgradeOrErr},
    ui::d2::{layout::Fill2D, widget::View},
    window::Window,
};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;
    let window = Window::new(&app)?;

    {
        let w = window.upgrade_or_err()?;
        let mut w = w.borrow_mut();
        w.set_title("View Example");

        let view = View::new();
        let mut fill_layout = Fill2D::new();
        fill_layout.set_widget(Box::new(view));

        w.set_layout(Box::new(fill_layout));
    }

    app.run()?;

    Ok(())
}
