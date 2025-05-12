use antiq::application::{Application, ApplicationError};

#[test]
fn create_multiple_apps() {
    let app1 = Application::new();
    assert_eq!(app1.is_ok(), true);

    let app2 = Application::new();
    assert!(matches!(app2, Err(ApplicationError::AlreadyExists)));
}
