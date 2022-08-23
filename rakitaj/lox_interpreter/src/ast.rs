use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(PartialEq, Debug)]
pub enum Expr {
    LiteralBool(bool),
    LiteralNumber(f32),
    LiteralNil,
    LiteralString(String),

    Grouping(Box<Expr>),

    Unary(TokenType, Box<Expr>),

    Binary(Box<Expr>, TokenType, Box<Expr>),
}

#[derive(PartialEq)]
pub struct Ast {
    pub root_expr: Expr
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, String),
    UnexpectedTokenType(TokenType, String),
    ExpressionIsNone
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn is_at_end(&self) -> bool {
        match self.tokens.get(self.current) {
            Some(token) => token.token_type == TokenType::Eof,
            None => true
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        match self.tokens.get(self.current) {
            Some(token) => token.token_type == *token_type,
            None => false
        }
    }

    fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        // while let Some(token) = self.tokens.get(self.current) {
        //     match token.token_type {
        //         TokenType::BangEqual | TokenType::EqualEqual => {
        //             let operator = self.tokens[self.current];
        //             self.current += 1;
        //             let right = self.comparison().unwrap();
        //             expr = Expr::Binary(Box::new(expr), operator.token_type, Box::new(right));
        //         },
        //         _ => {}
        //     }
        // }
        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token_type(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().token_type.clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().token_type.clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
          let operator = self.previous().token_type.clone();
          let right = self.unary();
          return Expr::Unary(operator, Box::new(right)
        );
        }
    
        return self.primary().unwrap();
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Option<Expr> = None;
        while let Some(token) = self.tokens.get(self.current) {
            match &token.token_type {
                TokenType::False => {
                    self.current += 1;
                    expr = Some(Expr::LiteralBool(false))
                },
                TokenType::True => {
                    self.current += 1;
                    expr = Some(Expr::LiteralBool(true))
                },
                TokenType::Nil => {
                    self.current += 1;
                    expr = Some(Expr::LiteralNil)
                },
                TokenType::Number(x) => {
                    self.current += 1;
                    expr = Some(Expr::LiteralNumber(*x))
                },
                TokenType::String(x) => {
                    self.current += 1;
                    expr = Some(Expr::LiteralString(x.to_string()))
                },
                TokenType::LeftParen => {
                    let expr_result = self.expression();
                    self.consume(&token.token_type, "Expect ')' after expression.");
                    expr = Some(Expr::Grouping(Box::new(expr_result)));
                }
                _ => return Err(ParseError::UnexpectedToken(token.clone(), "Found an unexpected token.".to_string()))
            }
        }
        return match expr {
            Some(x) => Ok(x),
            None => Err(ParseError::ExpressionIsNone)
        };
    }

        // if self.match_token_type(&[TokenType::False]) {
        //     return Expr::LiteralBool(false);
        // }
        // if self.match_token_type(&[TokenType::True]) {
        //     return Expr::LiteralBool(true);
        // }
        // if self.match_token_type(&[TokenType::Nil]) {
        //     return Expr::LiteralNil;
        // }
        // if self.match_token_type(&[TokenType::Number(1.0)]) {
        //     match self.tokens[self.current].token_type {
        //         TokenType::Number(n) => return Expr::LiteralNumber(n),
        //         _ => panic!("Index {index}\nToken type: {token_type}\n", index=self.current, token_type=self.tokens[self.current])
        //     }
        // }
        // if self.match_token_type(&[TokenType::String("".to_string())]) {
        //     match &self.tokens[self.current].token_type {
        //         TokenType::String(s) => return Expr::LiteralString(s.to_string()),
        //         _ => panic!("Index {index}\nToken type: {token_type}\n", index=self.current, token_type=self.tokens[self.current])
        //     }
        // }
        // if self.match_token_type(&[TokenType::LeftParen]) {
        //     let expr = self.expression();
        //     self.consume(&TokenType::RightParen, "Expect ')' after expression.");
        //     return Expr::Grouping(Box::new(expr));
        // }
        // panic!("Index {index}\nToken type: {token_type}\n", index=self.current, token_type=self.tokens[self.current]);
    

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<(), ParseError> {
        if self.check(token_type) {
            self.advance();
            return Ok(());
        } else {
            return Err(ParseError::UnexpectedTokenType(token_type.clone(), message.to_string()));
        }
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

    #[rstest]
    #[case(vec![], 0, true)]
    #[case(vec![Token::new(TokenType::LeftParen, 1)], 0, false)]
    #[case(vec![Token::new(TokenType::RightParen, 1)], 1, true)]
    #[case(vec![Token::new(TokenType::Eof, 1)], 0, true)]
    #[case(vec![Token::new(TokenType::LeftParen, 1), Token::new(TokenType::Eof, 1)], 1, true)]
    pub fn test_is_at_end(#[case] tokens: Vec<Token>, #[case] i: usize, #[case] expected: bool) {
        let mut parser = Parser::new(tokens);
        parser.current = i;
        assert_eq!(parser.is_at_end(), expected);
    }

    // #[test]
    // pub fn test_no_token_should_be_parse_error() {
    //     let tokens: Vec<Token> = Vec::new();
    //     let parser = Parser::new(tokens);
    //     assert_eq!(parser.parse(), Err(ParseError::ExpressionIsNone));
    // }

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
        let tokens = vec![
            Token::new(TokenType::True, 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::False, 1),
            Token::new(TokenType::Eof, 1)
        ];
        let expected_ast = 
            Expr::Binary(Box::new(
                Expr::LiteralBool(true)), 
                TokenType::EqualEqual, 
                Box::new(Expr::LiteralBool(false)));
        let mut parser = Parser::new(tokens);
        let actual_ast = parser.parse();
        assert_eq!(actual_ast, expected_ast);
    }

    // #[test]
    // fn test_math_expression_to_ast() {
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
    //     let expected_ast = 
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
    //             Box::new(Expr::LiteralNumber(9.0)));
    //     let parser = Parser::new(tokens);
    //     let actual_ast = Ast::new(parser);
    //     assert_eq!(actual_ast.root_expr, expected_ast);
    // }
}
