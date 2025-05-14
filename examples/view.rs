use antiq::{
    application::Application,
    core::{Result, UpgradeOrErr},
    ui::{layout::fill::Fill, widget::view::View},
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
        w.set_visible(true);

        let view = View::new();
        let mut fill_layout = Fill::new();
        fill_layout.set_widget(Box::new(view));

        w.set_layout(Box::new(fill_layout));
    }

    app.run()?;

    Ok(())
}
