use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

mod token;
use token::{SourceCode, lex};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 || args[0] == "/?" || args[0] == "--help" || args[0] == "-?" {
        println!("Usage: jlox [script]")
    } else if args.len() == 1 && args[0] == "prompt" {
        run_prompt();
    } else if args.len() == 1 {
        run_file(&args[0]);
    }
    
}

fn run_file(filepath: &str) -> () {
    let raw_source = load_source(filepath);
    run(raw_source)
    
}

fn run_prompt() -> () {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let unwrapped_line = match line {
            Ok(x) => x,
            Err(_) => break
        };
        if unwrapped_line.len() == 0 {
            break;
        } else {
            run(unwrapped_line);
        }
    }
}

fn run(raw_source: String) -> () {
    let source = SourceCode::new(raw_source);
    let tokens = lex(source);
    for token in tokens {
        println!("{:?}", token);
    }
}

fn load_source(filepath: &str) -> String {
    let result_contents = fs::read_to_string(filepath);
    match result_contents {
        Ok(contents) => contents,
        Err(err) => {
            error(0, err.to_string());
            panic!();
        }
    }
}

fn error(line: usize, message: String) -> () {
    report(line, "".to_string(), message);
}

fn report(line: usize, location: String, message: String) -> () {
    println!("[line {}] Error {}: {}", line, location, message);
}