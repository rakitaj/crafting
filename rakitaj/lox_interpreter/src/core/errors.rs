use super::location::Location;

#[derive(Debug, PartialEq, Eq)]
pub enum LoxError {
    SyntaxError(Location, String),
    RuntimeError(Location, String),
    Critical(String)
}

impl LoxError {
    pub fn new_syntax_error(location: Location, message: String) -> Self {
        LoxError::SyntaxError(location, message)
    }
}