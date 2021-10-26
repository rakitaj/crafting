use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

mod tokens;
mod scanner;
mod ast;
use ast::Ast;
use ast::Expr;
use scanner::SourceCode;
use tokens::Token;
use tokens::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();
    args.iter().for_each(|arg| println!("Argument: {}", arg));
    // Rust includes the path of the exe as the default 0th arg.
    if args.len() == 1 || args[1] == "/?" || args[1] == "--help" || args[1] == "-?" {
        print_help();
    } else if args.len() == 2 && args[1] == "prompt" {
        run_prompt();
    } else if args.len() == 3 && args[1] == "rlox" {
        run_file(&args[2]);
    } else {
        println!("Can't understand your args");
        print_help();
    }
}

fn print_help() {
    println!("Execute script          : rlox [script]");
    println!("Start interactive prompt: prompt");
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
    let mut source = SourceCode::new(raw_source);
    for token in source.scan_tokens() {
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

fn create_ast() -> Ast {
    let root = Ast {
        node: Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, 1),
                Box::new(Expr::LiteralNumber(123.0)))),
            Token::new(TokenType::Star, 1),
            Box::new(Expr::Grouping(Box::new(Expr::LiteralNumber(45.67))))
        )
    };
    return root;
}
