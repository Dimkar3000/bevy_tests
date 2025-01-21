use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct GameError(pub String);

impl GameError {
    pub fn new<'a>(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<std::io::Error> for GameError {
    fn from(value: std::io::Error) -> Self {
        GameError(value.to_string())
    }
}
