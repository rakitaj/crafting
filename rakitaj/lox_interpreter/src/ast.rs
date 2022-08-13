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

    fn peek(&self) -> Option<Token> {
        match self.tokens.get(self.current) {
            Some(token) => Some(*token.clone()),
            None => None
        }
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1];
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.token_type == TokenType::Eof,
            None => false
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        match self.peek() {
            Some(token) => token.token_type == *token_type,
            None => false
        }
    }

    fn match_token_type(&self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator.token_type, Box::new(right));
        }
        return expr;
    }

    fn comparison(&self) -> Expr {
        let mut expr = self.term();
        while self.match_token_type(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator.token_type, Box::new(right));
        }
        return expr;
    }

    fn term(&self) -> Expr {
        let mut expr = self.factor();
        while self.match_token_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator.token_type, Box::new(right));
        }
        return expr;
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();
        while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator.token_type, Box::new(right));
        }
        return expr;
    }

    fn unary(&self) -> Expr {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
          let operator = self.previous();
          let right = self.unary();
          return Expr::Unary(operator.token_type, Box::new(right)
        );
        }
    
        return self.primary();
    }

    fn primary(&self) -> Expr {
        if self.match_token_type(&[TokenType::False]) {
            return Expr::LiteralBool(false);
        }
        if self.match_token_type(&[TokenType::True]) {
            return Expr::LiteralBool(true);
        }
        if self.match_token_type(&[TokenType::Nil]) {
            return Expr::LiteralNil;
        }
        if self.match_token_type(&[TokenType::Number(0.0)]) {
            match self.tokens[self.current].token_type {
                TokenType::Number(n) => return Expr::LiteralNumber(n),
                _ => panic!()
            }
        }
        if self.match_token_type(&[TokenType::String("".to_string())]) {
            match self.tokens[self.current].token_type {
                TokenType::String(s) => return Expr::LiteralString(s),
                _ => panic!()
            }
        }
        if self.match_token_type(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.")
            return Expr::Grouping(Box::new(expr));
        }
        panic!();
    }

    fn consume(&self, token_type: TokenType, message: &str) {
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

    #[test]
    fn test_basic_equality_expr() {
        // true == false
        let token = &[
            Token::new(TokenType::True, 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::False, 1),
            Token::new(TokenType::Eof, 1)
        ];
        let expected_ast = Ast::new(
            Expr::Binary(Box::new(
                Expr::LiteralBool(true)), 
                TokenType::EqualEqual, 
                Box::new(Expr::LiteralBool(false)))
        );
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
