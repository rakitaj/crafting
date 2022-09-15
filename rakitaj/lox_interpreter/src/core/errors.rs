use super::location::Location;

#[derive(Debug, PartialEq)]
pub enum LoxError {
    SyntaxError(Location, String),
    RuntimeError(Location, String)
}

impl LoxError {
    pub fn new_syntax_error(location: Location, message: String) -> Self {
        LoxError::SyntaxError(location, message)
    }
}