use std::fs;

use crate::{scanner::SourceCode, parser::{Stmt, Parser}, core::errors::LoxError};

pub fn load_source(filepath: &str) -> String {
    let result_contents = fs::read_to_string(filepath);
    match result_contents {
        Ok(contents) => contents,
        Err(err) => {
            error(0, err.to_string());
            panic!();
        }
    }
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: usize, location: String, message: String) {
    println!("[line {}] Error {}: {}", line, location, message);
}

pub fn raw_source_to_ast(source: &str, filename: &str) -> Result<Vec<Stmt>, LoxError> {
    let mut source_code = SourceCode::new(source, filename.to_string());
    let tokens = source_code.scan_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn filename_to_ast(filepath: &str) -> Result<Vec<Stmt>, LoxError> {
    let raw_source = load_source(filepath);
    let mut source_code = SourceCode::new(&raw_source, filepath.to_string());
    let tokens = source_code.scan_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse()
}