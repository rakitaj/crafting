use lox_interpreter::{parser::{Expr, Literal}, tokens::TokenType};

#[test]
fn test_hardcoded_source_code_to_ast() {
    let s = "(1 + 2) / 3 == 1".to_string();
    let ast_result = lox_interpreter::parser::source_to_ast(s, "integration-test.lox".to_string());
    let expected_ast = Expr::Binary(
        Box::new(
        Expr::Binary(
            Box::new(
            Expr::Grouping(Box::new(Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(1.0))), 
                TokenType::Plus, 
                Box::new(Expr::Literal(Literal::Number(2.0)))
            )))),
            TokenType::Slash,
            Box::new(Expr::Literal(Literal::Number(3.0))))),
        TokenType::EqualEqual,
        Box::new(Expr::Literal(Literal::Number(1.0))));
    assert_eq!(ast_result.unwrap(), expected_ast)
}
