use antiq::application::Application;
use antiq::core::Result;
use antiq::window::Window;

#[test]
fn create_window() -> Result<()> {
    let app = Application::new()?;

    let title = "Test Window".to_string();

    let window = Window::new(&app)?
        .upgrade()
        .ok_or("Window weak reference is invalid")?;
    {
        window.borrow_mut().set_title(&title);
    }

    assert_eq!(window.borrow().title(), title);

    Ok(())
}
