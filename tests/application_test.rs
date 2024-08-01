use angie3d::Application;

#[test]
fn create_application() {
    let app1 = Application::new();
    let app2 = Application::new();

    assert_eq!(app1.is_ok(), true);
    assert_eq!(app2.is_err(), true);
}
