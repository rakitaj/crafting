use crate::core::location::Location;
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::core::errors::LoxError;

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
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Location, Literal),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token)
}

#[derive(PartialEq, Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>)
}

#[derive(PartialEq)]
pub struct Ast {
    pub root_expr: Expr
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }
        match statements.len() {
            0 => Err(LoxError::Critical("No tokens parsed.".to_string())),
            _ => Ok(statements)
        }
        
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

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token_type(&[TokenType::Var]) {
            match self.var_declaration() {
                Ok(x) => return Ok(x),
                Err(err) => {
                    self.synchronize();
                    return Err(err);
                }
            }
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume_identifier("Trying to consume an identifier as part of a var declaration.")?;      
        let cloned_name = name.clone();

        let mut initializer: Option<Expr> = None;
        if self.match_token_type(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }
        self.consume(&TokenType::SemiColon, "Expect ';' after variable declaration.")?;
        match initializer {
            Some(x) => Ok(Stmt::Var(cloned_name, x)),
            None => {
                // If there is no initializer reuse the location of the var identifier for the literal nil token that we infer.
                let nil_token_location = cloned_name.clone();
                Ok(Stmt::Var(cloned_name, Expr::Literal(nil_token_location.location, Literal::Nil)))
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token_type(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_token_type(&[TokenType::LeftBrace]) {
            let block = self.block()?;
            Ok(Stmt::Block(block))
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() && !self.check(&TokenType::RightBrace) {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        let _ = self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expect ; after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expect ; after value.")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;
        if self.match_token_type(&[TokenType::Equal]) {
            let location = self.previous().location.clone();
            let value = self.assignment()?;

            return match expr {
                Expr::Variable(token) => Ok(Expr::Assign(token, Box::new(value))),
                _ => Err(LoxError::RuntimeError(location, "Invalid assignment target".to_string()))
            }
        }
        Ok(expr)        
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;
        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;
        while self.match_token_type(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;
        while self.match_token_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;
        while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))

        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
          let operator = self.previous().clone();
          return match self.unary() {
            Ok(right) => Ok(Expr::Unary(operator, Box::new(right))),
            Err(err) => Err(err)
          }
        }
    
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Option<Expr> = None;

        let token = self.tokens.get(self.current)
            .ok_or_else(|| LoxError::SyntaxError(Location::Unknown, format!("Token get out of index i={}", self.current)))?;
        let location = token.location.clone();

        match &(token.token_type) {
            TokenType::False => {
                self.current += 1;
                expr = Some(Expr::Literal(location, Literal::False))
            },
            TokenType::True => {
                self.current += 1;
                expr = Some(Expr::Literal(location, Literal::True))
            },
            TokenType::Nil => {
                self.current += 1;
                expr = Some(Expr::Literal(location, Literal::Nil))
            },
            TokenType::Number(x) => {
                self.current += 1;
                expr = Some(Expr::Literal(location, Literal::Number(*x)))
            },
            TokenType::String(x) => {
                self.current += 1;
                expr = Some(Expr::Literal(location, Literal::String(x.to_string())))
            },
            TokenType::LeftParen => {
                self.current += 1;
                let expr_result = self.expression()?;
                self.consume(&TokenType::RightParen, "Expect ')' after expression. After the expression finishes parsing the next token type must be a RightParen.")?;
                expr = Some(Expr::Grouping(Box::new(expr_result)));
            },
            TokenType::Identifier(_) => {
                self.current += 1;
                //expr = Some(Expr::Variable(self.previous().clone()))
                expr = Some(Expr::Variable(token.clone()))
            }
            _ => {}
        }
        match expr {
            Some(x) => Ok(x),
            None => {
                // Why do I have to get the token from self and can't use token.location.clone()
                let t = self.tokens.get(self.current).unwrap();
                Err(LoxError::RuntimeError(t.location.clone(), "Expected expression and found None.".to_string()))
            }
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, LoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let unexpected_token = self.tokens[self.current].clone();
            let msg = format!("{message}\nUnexpected token is {unexpected_token}");
            Err(LoxError::SyntaxError(unexpected_token.location, msg))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<&Token, LoxError> {
        if let Some(token) = self.tokens.get(self.current) {
            if let TokenType::Identifier(_) = &token.token_type {
                self.current += 1;
                return Ok(token);
            }
        }
        let unexpected_token = self.tokens[self.current].clone();
        let msg = format!("Tried to consume an identifier.\nUnexpected token is {unexpected_token}\n{message}");
        Err(LoxError::SyntaxError(unexpected_token.location, msg))
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

pub fn parenthesize_statements(statements: &[Stmt]) -> String {
    let mut strings: Vec<String> = Vec::new();
    for stmt in statements {
        if let Stmt::Expression(expr) = stmt {
            strings.push(parenthesize(expr));
        }
    }
    strings.join("\n")
}

pub fn parenthesize(expr: &Expr) -> String {
    match expr {
        Expr::Literal(_, Literal::False) => "false".to_string(),
        Expr::Literal(_, Literal::True) => "true".to_string(),
        Expr::Literal(_, Literal::Nil) => "nil".to_string(),
        Expr::Literal(_, Literal::Number(value)) => value.to_string(),
        Expr::Literal(_, Literal::String(value)) => value.to_string(),
        Expr::Grouping(expr) => format!("(group {})", parenthesize(expr)),
        Expr::Unary(token, expr) => format!("({} {})", token.token_type, parenthesize(expr)),
        Expr::Binary(expr_left, token, expr_right) => format!("({} {} {})", token.token_type, parenthesize(expr_left), parenthesize(expr_right)),
        Expr::Ternary(expr_conditional, expr_left, expr_right) => format!("(ternary {} {} {})", parenthesize(expr_conditional), parenthesize(expr_left), parenthesize(expr_right)),
        Expr::Variable(var_identifier) => format!("var {}", var_identifier),
        Expr::Assign(token, expr) => format!("({} = {})", token, parenthesize(expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn loc(line: usize) -> Location {
        Location::Line("unittest.lox".to_string(), line)
    }

    #[rstest]
    #[case(vec![], 0, true)]
    #[case(vec![Token::new(TokenType::LeftParen, loc(1))], 0, false)]
    #[case(vec![Token::new(TokenType::RightParen, loc(1))], 1, true)]
    #[case(vec![Token::new(TokenType::Eof, loc(1))], 0, true)]
    #[case(vec![Token::new(TokenType::LeftParen, loc(1)), Token::new(TokenType::Eof, loc(1))], 1, true)]
    fn test_is_at_end(#[case] tokens: Vec<Token>, #[case] i: usize, #[case] expected: bool) {
        let mut parser = Parser::new(tokens);
        parser.current = i;
        assert_eq!(parser.is_at_end(), expected);
    }

    #[test]
    fn test_no_token_should_be_critical_error() {
        let tokens: Vec<Token> = Vec::new();
        let mut parser = Parser::new(tokens);
        let error = LoxError::Critical("No tokens parsed.".to_string());
        let actual_error = parser.parse().unwrap_err();
        assert_eq!(actual_error, error);
    }

    #[test]
    fn test_parenthesize() {
        // -123 * (45.67)
        let root_expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, loc(1)), 
                Box::new(Expr::Literal(loc(1), Literal::Number(123.0))))),
            Token::new(TokenType::Star, loc(1)),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(loc(1), Literal::Number(45.67))))));
        let result = parenthesize(&root_expr);
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }

    #[test]
    fn test_basic_equality_expr() {
        // true == false
        let tokens = vec![
            Token::new(TokenType::True, loc(1)),
            Token::new(TokenType::EqualEqual, loc(1)),
            Token::new(TokenType::False, loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1)),
        ];

        let expected_ast = 
            vec![Stmt::Expression(Expr::Binary(Box::new(
                Expr::Literal(loc(1), Literal::True)), 
                Token::new(TokenType::EqualEqual, loc(1)), 
                Box::new(Expr::Literal(loc(1), Literal::False))))];

        let mut parser = Parser::new(tokens);
        let actual_ast = parser.parse();
        assert_eq!(actual_ast.unwrap(), expected_ast);
    }

    #[test]
    fn test_basic_grouping_expr() {
        // (1 + 2)
        let tokens = vec![
            Token::new(TokenType::LeftParen, loc(1)),
            Token::new(TokenType::Number(1f32), loc(1)),
            Token::new(TokenType::Plus, loc(1)),
            Token::new(TokenType::Number(2f32), loc(1)),
            Token::new(TokenType::RightParen, loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1)),
        ];

        let expected_ast = 
            vec![Stmt::Expression(Expr::Grouping(Box::new(
                Expr::Binary(
                    Box::new(Expr::Literal(loc(1), Literal::Number(1.0))), 
                    Token::new(TokenType::Plus, loc(1)), 
                    Box::new(Expr::Literal(loc(1), Literal::Number(2.0)))
            ))))];
            
        let mut parser = Parser::new(tokens);
        let actual_ast = parser.parse();
        assert_eq!(actual_ast.unwrap(), expected_ast);
    }

    #[test]
    fn test_math_expression_to_ast() {
        // (1 + 2) * 3 == 9
        let tokens = vec![
            Token::new(TokenType::LeftParen, loc(1)),
            Token::new(TokenType::Number(1f32), loc(1)),
            Token::new(TokenType::Plus, loc(1)),
            Token::new(TokenType::Number(2f32), loc(1)),
            Token::new(TokenType::RightParen, loc(1)),
            Token::new(TokenType::Star, loc(1)),
            Token::new(TokenType::Number(3f32), loc(1)),
            Token::new(TokenType::EqualEqual, loc(1)),
            Token::new(TokenType::Number(9f32), loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ];

        let expected_ast = 
            vec![Stmt::Expression(Expr::Binary(
                Box::new(
                Expr::Binary(
                    Box::new(
                    Expr::Grouping(Box::new(Expr::Binary(
                        Box::new(Expr::Literal(loc(1), Literal::Number(1.0))), 
                        Token::new(TokenType::Plus, loc(1)), 
                        Box::new(Expr::Literal(loc(1), Literal::Number(2.0)))
                    )))),
                    Token::new(TokenType::Star, loc(1)),
                    Box::new(Expr::Literal(loc(1), Literal::Number(3.0))))),
                Token::new(TokenType::EqualEqual, loc(1)),
                Box::new(Expr::Literal(loc(1), Literal::Number(9.0)))))];
            
        let mut parser = Parser::new(tokens);
        let actual_ast = parser.parse();
        assert_eq!(actual_ast.unwrap(), expected_ast);
    }
}
