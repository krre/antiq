use antiq::core::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = Application::new()?;
    app.run();

    Ok(())
}
