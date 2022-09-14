pub enum LoxError {
    SyntaxError(Location, String),
    RuntimeError
}