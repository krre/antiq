use antiq::application::Application;
use antiq::window::Window;

#[test]
fn create_window() {
    let app = Application::new().unwrap();

    let title = "Test Window".to_string();

    let window = Window::new(&app).unwrap().upgrade().unwrap();
    window.set_title(&title);

    assert_eq!(window.title(), title);
}
