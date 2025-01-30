use std::fmt::Display;

#[derive(Debug)]
pub enum ApplicationError {
    AlreadyExists,
    Other(Box<dyn std::error::Error>),
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::AlreadyExists => write!(f, "application instance already exists"),
            ApplicationError::Other(e) => write!(f, "other application error: {}", e),
        }
    }
}

impl std::error::Error for ApplicationError {}
