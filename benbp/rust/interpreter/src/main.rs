mod iter_extensions;
mod scan;
mod token;
mod ast;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let program = fs::read_to_string(filename).expect(&format!("Error reading file {}", filename));

    if scan::tokenize(program) {
        std::process::exit(65);
    }
}
