use std::fmt;
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

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::SyntaxError(location, msg) => write!(f, "Syntax Error\n{}\nLocation @ {}", msg, location),
            LoxError::RuntimeError(location, msg) => write!(f, "Runtime Error\n{}\nLocation @ {}", msg, location),
            LoxError::Critical(msg) => write!(f, "\nCritical Error\n{}\nNo location can be determined. Something really bad happened.", msg)
        }
    }
}