use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Location {
    Unknown,
    Eof(String),
    Line(String, usize),
}

impl Location {
    pub fn new_line(filename: String, line: usize) -> Self {
        Location::Line(filename, line)
    }

    pub fn new_eof(filename: String) -> Self {
        Location::Eof(filename)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Unknown => f.write_str("Unknown location."),
            Location::Eof(filename) => {
                let message = format!("Error at -> {filename} end of file.", filename = filename);
                f.write_str(&message)
            }
            Location::Line(filename, line) => {
                let message = format!(
                    "Error at -> {filename}:{line}",
                    filename = filename,
                    line = line
                );
                f.write_str(&message)
            }
        }
    }
}
