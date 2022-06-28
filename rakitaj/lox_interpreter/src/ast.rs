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

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens, current: 0 }
    }

    fn advance(&mut self) -> Option<Token> {

    }

    fn match_token_type(&self, token_types: &[&TokenType]) -> bool {
        for token_type in token_types {
            if check(token_type) {
                advance();
                return true;
            }
        }
        return false;
    }

    pub fn expression(&self) -> Expr {
        self.equality()
    }

    pub fn equality(&self) -> Expr {
        let mut expr: Expr = self.comparison();
        while &self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = &self.revious();
            let right: Expr = &self.comparison();
            expr = Expr::Binary(expr, operator, right);
        }
        return expr;
    }
}

pub fn parenthesize(expr: Expr) -> String {
    match expr {
        Expr::LiteralBool(value) => match value {
            true => "true".to_string(),
            false => "false".to_string()
        },
        Expr::LiteralNumber(value) => value.to_string(),
        Expr::LiteralNil => "nil".to_string(),
        Expr::LiteralString(value) => value,
        Expr::Grouping(expr) => format!("(group {})", parenthesize(*expr)),
        Expr::Unary(token, expr) => format!("({} {})", token, parenthesize(*expr)),
        Expr::Binary(expr_left, token, expr_right) => format!("({} {} {})", token, parenthesize(*expr_left), parenthesize(*expr_right)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_parenthesize() {
        // -123 * (45.67)
        let root_expr = Expr::Binary(
            Box::new(Expr::Unary(
                TokenType::Minus, 
                Box::new(Expr::LiteralNumber(123.0)))),
            TokenType::Star,
            Box::new(Expr::Grouping(Box::new(Expr::LiteralNumber(45.67)))));
        let result = parenthesize(root_expr);
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }

    //#[test]
    // fn test_simple_expression_to_ast() {
    //     // (1 + 2) * 3 == 9
    //     let tokens = vec![
    //         Token::new(TokenType::LeftParen, 1),
    //         Token::new(TokenType::Number(1f32), 1),
    //         Token::new(TokenType::Plus, 1),
    //         Token::new(TokenType::Number(2f32), 1),
    //         Token::new(TokenType::RightParen, 1),
    //         Token::new(TokenType::Star, 1),
    //         Token::new(TokenType::Number(3f32), 1),
    //         Token::new(TokenType::EqualEqual, 1),
    //         Token::new(TokenType::Number(9f32), 1),
    //         Token::new(TokenType::Eof, 1)
    //     ];
    //     let ast = Ast::new(
    //         Expr::Binary(
    //             Box::new(
    //             Expr::Binary(
    //                 Box::new(
    //                 Expr::Grouping(Box::new(Expr::Binary(
    //                     Box::new(Expr::LiteralNumber(1.0)), 
    //                     TokenType::Plus, 
    //                     Box::new(Expr::LiteralNumber(2.0))
    //                 )))),
    //                 TokenType::Star,
    //                 Box::new(Expr::LiteralNumber(3.0)))),
    //             TokenType::EqualEqual,
    //             Box::new(Expr::LiteralNumber(9.0))));
    // }
}
