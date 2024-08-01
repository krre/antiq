use std::error::Error;

use angie3d::Application;

fn main() -> Result<(), Box<dyn Error>> {
    let app = Application::new()?;
    let _ = app.create_window();
    app.run();

    Ok(())
}
