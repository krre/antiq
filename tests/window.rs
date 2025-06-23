use antiq::application::Application;
use antiq::core::Result;
use antiq::window::Window;

#[test]
fn create_window() -> Result<()> {
    let app = Application::new()?;

    let title = "Test Window".to_string();

    let mut window = Window::new(&app)?;
    window.set_title(&title);

    assert_eq!(window.title(), title);

    Ok(())
}
