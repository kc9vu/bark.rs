use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct LackError {
    message: String,
}

impl LackError {
    pub fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for LackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LackError {}
