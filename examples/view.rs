use antiq::{
    application::Application,
    core::Result,
    ui::d2::{layout::Fill2D, widget::View},
    window::Window,
};

fn main() -> Result<()> {
    env_logger::init();

    let app = Application::new()?;

    let mut window = Window::new(&app)?;
    window.set_title("View Example");

    let view = View::new();
    let mut fill_layout = Fill2D::new();
    fill_layout.set_widget(Box::new(view));

    window.set_layout(Box::new(fill_layout));

    app.run()?;

    Ok(())
}
