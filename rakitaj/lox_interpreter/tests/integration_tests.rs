use lox_interpreter::{parser::{Expr, Literal, Parser, Stmt, ParseResult}, tokens::{TokenType, Token}, core::{location::Location, errors::LoxError}, scanner::SourceCode, interpreter::{InterpreterState, Interpreter}};

fn loc(line: usize) -> Location {
    Location::Line("integration-test.lox".to_string(), line)
}

pub fn source_to_ast(source: &str, filename: String) -> Result<Vec<Stmt>, LoxError> {
    let mut source_code = SourceCode::new(source, filename);
    let tokens = source_code.scan_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn test_expression_to_ast() {
    let s = "(1 + 2) / 3 == 1;";
    let ast_result = source_to_ast(s, "integration-test.lox".to_string());
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
    let ast_result = source_to_ast(s, "integration-test.lox".to_string());
    let ast = vec![
        Stmt::Var(Token::new(TokenType::Identifier("foo".to_string()), loc(1)), Expr::Literal(loc(1), Literal::Nil))
    ];
    assert_eq!(ast_result, Ok(ast));
}

#[test]
fn test_conditional() {
    let filename = "conditional.lox";
    let mut source = SourceCode::new(filename, "repl.lox".to_string());
    let tokens = source.scan_tokens();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().must();
    let state = &mut InterpreterState::<Vec<u8>>::default();
    let interpreter = Interpreter::new(ast);
    interpreter.interpret(state);
    //assert_eq!(ast_result, Ok(ast));
}