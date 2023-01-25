use std::{io::{BufRead, self}, fs::File};

use rstest::*;
use lox_interpreter::{parser::{Expr, Literal, Stmt, ParseResult}, tokens::{TokenType, Token}, core::{location::Location}, interpreter::{InterpreterState, Interpreter}, runhelpers::{raw_source_to_ast, filepath_to_ast}};

fn loc(line: usize) -> Location {
    Location::Line("integration-test.lox".to_string(), line)
}

#[test]
fn test_expression_to_ast() {
    let s = "(1 + 2) / 3 == 1;";
    let ast_result = raw_source_to_ast(s, "integration-test.lox");
    let ast = 
    vec![Stmt::Expression(Expr::Binary(
        Box::new(
        Expr::Binary(
            Box::new(
            Expr::Grouping(Box::new(Expr::Binary(
                Box::new(Expr::Literal(loc(1), Literal::Number(1.0))), 
                Token::new(TokenType::Plus, loc(1)), 
                Box::new(Expr::Literal(loc(1), Literal::Number(2.0)))
            )))),
            Token::new(TokenType::Slash, loc(1)),
            Box::new(Expr::Literal(loc(1), Literal::Number(3.0))))),
        Token::new(TokenType::EqualEqual, loc(1)),
        Box::new(Expr::Literal(loc(1), Literal::Number(1.0)))))];
    assert_eq!(ast_result, Ok(ast))
}

#[test]
fn test_variable_declaration() {
    let s = "var foo;";
    let ast_result = raw_source_to_ast(s, "integration-test.lox");
    let ast = vec![
        Stmt::Var(Token::new(TokenType::Identifier("foo".to_string()), loc(1)), Expr::Literal(loc(1), Literal::Nil))
    ];
    assert_eq!(ast_result, Ok(ast));
}

fn expected_output(file_path: &str) -> String {
    let file = File::open(file_path).unwrap();
    let contents = io::BufReader::new(file).lines();
    let mut result = Vec::new();
    // Consumes the iterator, returns an (Optional) String
    for line in contents {
        let l = line.unwrap();
        if l.starts_with("//") {
            let parts: Vec<&str> = l.split("//").collect();
            let expected = parts[1][1..].to_string();
            result.push(expected + "\n")
        } else {
            break
        }
    }
    return result.join("");
}

#[rstest]
#[case("conditional-and-false.lox")]
#[case("conditional-and-true.lox")]
#[case("conditional-false.lox")]
#[case("conditional-true.lox")]
#[case("while-loop.lox")]
fn test_program_output_is_expected(#[case] filename: &str) {
    let filepath = format!("./data/{}", filename);
    let expected = expected_output(&filepath);
    let ast = filepath_to_ast(&filepath).must();
    let state = &mut InterpreterState::<Vec<u8>>::default();
    let interpreter = Interpreter::new(ast);
    interpreter.interpret(state);
    assert_eq!(state.get_writer(), expected)
}