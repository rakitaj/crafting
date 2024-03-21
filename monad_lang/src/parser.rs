use camino::Utf8PathBuf;
use crate::monad::Instruction;
use std::fs;

fn parse(file_path: Utf8PathBuf) -> Vec<Instruction> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.lines();
}

fn parse(line: &str) -> Instruction {
    
}