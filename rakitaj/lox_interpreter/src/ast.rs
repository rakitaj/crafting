use crate::tokens::Token;
use crate::tokens::TokenType;


pub enum Expr {
    LiteralBool(bool),
    LiteralNumber(f32),
    LiteralNil,
    LiteralString(String),

    Grouping(Box<Expr>),

    Unary(TokenType, Box<Expr>),

    Binary(Box<Expr>, TokenType, Box<Expr>),
}

pub struct Ast {
    pub node: Expr
}

impl Ast {
    pub fn new(expr: Expr) -> Self {
        Ast {
            node: expr
        }
    }
}

// pub fn tokens_to_ast(tokens: Vec<Token>) -> Ast {
//     while !tokens.is_empty() {

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_simple_expression_to_ast() {
        // (1 + 2) * 3 == 9
        let tokens = vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::Number(1f32), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Number(2f32), 1),
            Token::new(TokenType::RightParen, 1),
            Token::new(TokenType::Star, 1),
            Token::new(TokenType::Number(3f32), 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::Number(9f32), 1),
            Token::new(TokenType::Eof, 1)
        ];
        let ast = Ast::new(
            Expr::Binary(
                Box::new(
                Expr::Binary(
                    Box::new(
                    Expr::Grouping(Box::new(Expr::Binary(
                        Box::new(Expr::LiteralNumber(1.0)), 
                        TokenType::Plus, 
                        Box::new(Expr::LiteralNumber(2.0))
                    )))),
                    TokenType::Star,
                    Box::new(Expr::LiteralNumber(3.0)))),
                TokenType::EqualEqual,
                Box::new(Expr::LiteralNumber(9.0))));
    }
}
