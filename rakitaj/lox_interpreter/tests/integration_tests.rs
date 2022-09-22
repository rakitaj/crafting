use lox_interpreter::{parser::{Expr, Literal}, tokens::{TokenType, Token}, core::location::Location};

fn loc(line: usize) -> Location {
    Location::Line("integration-test.lox".to_string(), line)
}

#[test]
fn test_hardcoded_source_code_to_ast() {
    let s = "(1 + 2) / 3 == 1";
    let ast_result = lox_interpreter::parser::source_to_ast(s, "integration-test.lox".to_string());
    let expected_ast = Expr::Binary(
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
        Box::new(Expr::Literal(loc(1), Literal::Number(1.0))));
    assert_eq!(ast_result.unwrap(), expected_ast)
}
