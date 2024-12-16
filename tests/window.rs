use antiq::core::Application;
use antiq::core::Window;
use antiq::core::WindowSettings;

#[test]
fn create_window() {
    let app = Application::new().unwrap();

    let title = "Window title".to_string();

    let window = Window::new(app.context().clone(), WindowSettings::new()).unwrap();
    window.set_title(&title);

    assert_eq!(window.title(), title);
}
