use crate::scanner::SourceCode;
use crate::tokens::Token;
use crate::tokens::TokenType;

#[derive(PartialEq, Debug)]
pub enum Literal {
    Nil,
    Number(f32),
    String(String),
    True,
    False
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Literal(Literal),

    Grouping(Box<Expr>),

    Unary(TokenType, Box<Expr>),

    Binary(Box<Expr>, TokenType, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>)
}

#[derive(PartialEq)]
pub struct Ast {
    pub root_expr: Expr
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, String),
    UnexpectedTokenType(TokenType, String),
    ExpectExpression(usize),
    IndexError(usize)
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
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
        self.previous()
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
        false
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_token_type(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_token_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().token_type.clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().token_type.clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))

        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
          let operator = self.previous().token_type.clone();
          return match self.unary() {
            Ok(right) => Ok(Expr::Unary(operator, Box::new(right))),
            Err(err) => Err(err)
          }
        }
    
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Option<Expr> = None;

        let token = self.tokens.get(self.current)
            .ok_or(ParseError::IndexError(self.current))?;

        match &token.token_type {
            TokenType::False => {
                self.current += 1;
                expr = Some(Expr::Literal(Literal::False))
            },
            TokenType::True => {
                self.current += 1;
                expr = Some(Expr::Literal(Literal::True))
            },
            TokenType::Nil => {
                self.current += 1;
                expr = Some(Expr::Literal(Literal::Nil))
            },
            TokenType::Number(x) => {
                self.current += 1;
                expr = Some(Expr::Literal(Literal::Number(*x)))
            },
            TokenType::String(x) => {
                self.current += 1;
                expr = Some(Expr::Literal(Literal::String(x.to_string())))
            },
            TokenType::LeftParen => {
                self.current += 1;
                let expr_result = self.expression()?;
                // If the Result type is an error, use the ? to short-circuit and return it early.
                let _ = self.consume(&TokenType::RightParen, "Expect ')' after expression. After the expression finishes parsing the next token type must be a RightParen.")?;
                expr = Some(Expr::Grouping(Box::new(expr_result)));
            }
            _ => {}
        }
        match expr {
            Some(x) => Ok(x),
            None => Err(ParseError::ExpectExpression(self.current))
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<(), ParseError> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            let unexpected_token = self.tokens[self.current].clone();
            Err(ParseError::UnexpectedToken(unexpected_token, message.to_string()))
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return
            }

            match self.tokens[self.current].token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var
                | TokenType:: For | TokenType::If | TokenType:: While
                | TokenType::Print | TokenType::Return => return,
                _ => { self.advance(); }
            }
        }
    }
}

pub fn source_to_ast(source: String) -> Result<Expr, ParseError> {
    let mut source_code = SourceCode::new(source);
    let tokens = source_code.scan_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn parenthesize(expr: Expr) -> String {
    match expr {
        Expr::Literal(Literal::False) => "false".to_string(),
        Expr::Literal(Literal::True) => "true".to_string(),
        Expr::Literal(Literal::Nil) => "nil".to_string(),
        Expr::Literal(Literal::Number(value)) => value.to_string(),
        Expr::Literal(Literal::String(value)) => value,
        Expr::Grouping(expr) => format!("(group {})", parenthesize(*expr)),
        Expr::Unary(token, expr) => format!("({} {})", token, parenthesize(*expr)),
        Expr::Binary(expr_left, token, expr_right) => format!("({} {} {})", token, parenthesize(*expr_left), parenthesize(*expr_right)),
        Expr::Ternary(expr_conditional, expr_left, expr_right) => format!("(ternary {} {} {})", parenthesize(*expr_conditional), parenthesize(*expr_left), parenthesize(*expr_right))
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
    fn test_is_at_end(#[case] tokens: Vec<Token>, #[case] i: usize, #[case] expected: bool) {
        let mut parser = Parser::new(tokens);
        parser.current = i;
        assert_eq!(parser.is_at_end(), expected);
    }

    #[test]
    pub fn test_no_token_should_be_parse_error() {
        let tokens: Vec<Token> = Vec::new();
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParseError::IndexError(0)));
    }

    #[test]
    fn test_parenthesize() {
        // -123 * (45.67)
        let root_expr = Expr::Binary(
            Box::new(Expr::Unary(
                TokenType::Minus, 
                Box::new(Expr::Literal(Literal::Number(123.0))))),
            TokenType::Star,
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(45.67))))));
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
                Expr::Literal(Literal::True)), 
                TokenType::EqualEqual, 
                Box::new(Expr::Literal(Literal::False)));
        let mut parser = Parser::new(tokens);
        let actual_ast = parser.parse();
        assert_eq!(actual_ast.unwrap(), expected_ast);
    }

    #[test]
    fn test_basic_grouping_expr() {
        // (1 + 2)
        let tokens = vec![
            Token::new(TokenType::LeftParen, 1),
            Token::new(TokenType::Number(1f32), 1),
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::Number(2f32), 1),
            Token::new(TokenType::RightParen, 1)
        ];
        let expected_ast = 
            Expr::Grouping(Box::new(
                Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(1.0))), 
                    TokenType::Plus, 
                    Box::new(Expr::Literal(Literal::Number(2.0)))
            )));
        let mut parser = Parser::new(tokens);
        let actual_ast = parser.parse();
        assert_eq!(actual_ast.unwrap(), expected_ast);
    }

    #[test]
    fn test_math_expression_to_ast() {
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
        let expected_ast = 
            Expr::Binary(
                Box::new(
                Expr::Binary(
                    Box::new(
                    Expr::Grouping(Box::new(Expr::Binary(
                        Box::new(Expr::Literal(Literal::Number(1.0))), 
                        TokenType::Plus, 
                        Box::new(Expr::Literal(Literal::Number(2.0)))
                    )))),
                    TokenType::Star,
                    Box::new(Expr::Literal(Literal::Number(3.0))))),
                TokenType::EqualEqual,
                Box::new(Expr::Literal(Literal::Number(9.0))));
        let mut parser = Parser::new(tokens);
        let actual_ast = parser.parse();
        assert_eq!(actual_ast.unwrap(), expected_ast);
    }
}