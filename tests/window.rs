use antiq::application::Application;
use antiq::core::{Result, UpgradeOrErr};
use antiq::window::Window;

#[test]
fn create_window() -> Result<()> {
    let app = Application::new()?;

    let title = "Test Window".to_string();

    let window = Window::new(&app)?.upgrade_or_err()?;
    {
        window.borrow_mut().set_title(&title);
    }

    assert_eq!(window.borrow().title(), title);

    Ok(())
}
