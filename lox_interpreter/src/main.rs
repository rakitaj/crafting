use std::env;
use std::io;
use std::io::prelude::*;

use lox_interpreter::interpreter::Interpreter;
use lox_interpreter::interpreter::InterpreterState;
use lox_interpreter::parser::parenthesize_statements;
use lox_interpreter::runhelpers::load_source;
use lox_interpreter::scanner::SourceCode;
use lox_interpreter::parser::Parser;

enum ReplMode {
    Standard,
    Debug
}

fn main() {
    let args: Vec<String> = env::args().collect();
    args.iter().for_each(|arg| println!("Argument: {}", arg));
    // Rust includes the path of the exe as the default 0th arg.
    if args.len() == 1 || args[1] == "/?" || args[1] == "--help" || args[1] == "-?" {
        print_help();
    } else if args.len() == 2 && args[1] == "prompt" {
        run_prompt(ReplMode::Standard);
    } else if args.len() == 2 && args[1] == "debug-prompt" {
        run_prompt(ReplMode::Debug);
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
    println!("Start interactive language dev prompt: debug-prompt");
}

fn run_file(filepath: &str) {
    let raw_source = load_source(filepath);
    run(raw_source);
    
}

fn run_prompt(mode: ReplMode) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let unwrapped_line = match line {
            Ok(x) => x,
            Err(_) => break
        };
        if unwrapped_line.is_empty() {
            break;
        } else {
            match mode {
                ReplMode::Standard => run(unwrapped_line),
                ReplMode::Debug => run_debug(unwrapped_line)
            }
        }
    }
}

fn run(raw_source: String) {
    let mut source = SourceCode::new(&raw_source, "repl.lox".to_string());
    let tokens = source.scan_tokens();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            let state = &mut InterpreterState::<std::io::Stdout>::default();
            let interpreter = Interpreter::new(ast);
            interpreter.interpret(state);
        },
        Err(err) => println!("Lox error:\n{}", err)
    }
}

fn run_debug(raw_source: String) {
    let mut source = SourceCode::new(&raw_source, "repl.lox".to_string());
    let tokens = source.scan_tokens();
    
    for token in &tokens {
        println!("{:?}", token);
    }
    
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("{}", parenthesize_statements(&ast));
            let state = &mut InterpreterState::<std::io::Stdout>::default();
            let interpreter = Interpreter::new(ast);
            interpreter.interpret(state);
        },
        Err(err) => println!("Lox error:\n{}", err)
    }
}



