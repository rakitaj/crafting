use std::env;
use std::fs;

mod token;
use token::{SourceCode};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args[0] == "/?" || args[0] == "--help" || args[0] == "-?" {
        println!("Give the filepath as the first and only argument.")
    }
    let raw_source = load_source(&args[0]);
    let source = SourceCode::new(raw_source);
}

fn load_source(filepath: &str) -> String {
    let result_contents = fs::read_to_string(filepath);
    match result_contents {
        Ok(contents) => contents,
        Err(error) => panic!("Probably can't find the file {:?}", error)
    }
}
