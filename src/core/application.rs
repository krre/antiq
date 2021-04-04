pub fn name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
