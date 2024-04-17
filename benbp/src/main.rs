mod ast;
mod iter_extensions;
mod scan;
mod token;

use std::env;
use std::fs;
use token::Literal::Number;
use token::Token;
use token::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    if filename == "print" {
        test_ast_printer();
        return;
    }

    let program = fs::read_to_string(filename).unwrap_or_else(|_| panic!("Error reading file {}", filename));
    
    if scan::tokenize(program) {
        std::process::exit(65);
    }
}

fn test_ast_printer() {
    let expression = ast::Expr::Binary(
        Box::new(ast::Expr::Unary(
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                line: 0,
                literal: Some(Number(1.0)),
            },
            Box::new(ast::Expr::Literal(Number(123.0))),
        )),
        Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            line: 0,
            literal: Some(Number(1.0)),
        },
        Box::new(ast::Expr::Grouping(Box::new(ast::Expr::Literal(
            Number(45.67),
        )))),
    );
    println!("{}", ast::format_ast(expression));
}
