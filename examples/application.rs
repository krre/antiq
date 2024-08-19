use std::error::Error;

use angie3d::Application;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = Application::new()?;
    app.run()?;

    Ok(())
}
