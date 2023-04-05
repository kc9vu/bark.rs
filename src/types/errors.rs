use std::{error::Error, fmt::{self, Display, Formatter}};

#[derive(Debug, Clone)]
pub struct LackError <'a> {
    message: &'a str,
}

impl <'a> LackError <'a> {
    pub fn from (message: &'a str) -> Self {
        Self { message }
    }
}

impl <'a> Display for LackError <'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl <'a> Error for LackError <'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct NotExistsError <'a> {
    pub path: &'a str,
}

impl <'a> NotExistsError <'a> {
    pub fn from(path: &'a str) -> Self {
        Self { path }
    }
}

impl <'a> Display for NotExistsError <'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
impl <'a> Error for NotExistsError <'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}