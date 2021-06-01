use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    WHile,

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: usize
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize
    ) -> Self {
        Token {
            token_type: token_type,
            line: line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{} {}", &self.token_type, &self.line
        )
    }
}

pub struct SourceCode {
    pub source: String,
    pub index: usize,
    pub line: usize,
}

impl SourceCode {
    pub fn new(source: String) -> Self {
        SourceCode {
            source: source,
            index: 0,
            line: 1,
        }
    }

    pub fn get(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }

    pub fn peek(&self, n: usize) -> Option<char> {
        self.source.chars().nth(&self.index + n)
    }

    pub fn get_string(&self, n: usize) -> String {
        self.source[self.index..self.index + n].to_string()
    }

    pub fn eof(&self) -> bool {
        match &self.get() {
            Some(_) => false,
            None => true,
        }
    }

    pub fn advance_to_eol(&mut self) -> () {
        while let Some(c) = self.get() {
            match c {
                '\n' => { self.line += 1; break }
                _ => self.index += 1
            }
        }
    }

    pub fn scan_string_literal(&mut self) -> String {
        "foo".to_string()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.get() {
            match c {
                ' ' => {},
                '\t' => {},
                '\n' => {
                    self.line += 1;
                }
                '(' => tokens.push(Token::new(TokenType::LeftParen, self.line)),
                ')' => tokens.push(Token::new(TokenType::RightParen, self.line)),
                '{' => tokens.push(Token::new(TokenType::LeftBrace, self.line)),
                '}' => tokens.push(Token::new(TokenType::RightBrace, self.line)),
                ',' => tokens.push(Token::new(TokenType::Comma, self.line)),
                '.' => tokens.push(Token::new(TokenType::Dot, self.line)),
                '-' => tokens.push(Token::new(TokenType::Minus, self.line)),
                '+' => tokens.push(Token::new(TokenType::Plus, self.line)),
                ';' => tokens.push(Token::new(TokenType::SemiColon, self.line)),
                '*' => tokens.push(Token::new(TokenType::Star, self.line)),
                '/' => {
                    match self.peek(1) {
                        Some('/') => { self.advance_to_eol() },
                        _ => tokens.push(Token::new(TokenType::Slash, self.line))
                    }
                },
                '!' => {
                    match self.peek(1) {
                        Some('=') => { tokens.push(Token::new(TokenType::BangEqual, self.line)); self.index += 1; },
                        _ => tokens.push(Token::new(TokenType::Bang, self.line))
                    }
                },
                '=' => {
                    match self.peek(1) {
                        Some('=') => { tokens.push(Token::new(TokenType::EqualEqual, self.line)); self.index += 1; },
                        _ => tokens.push(Token::new(TokenType::Equal, self.line))
                    }
                },
                '<' => {
                    match self.peek(1) {
                        Some('=') => { tokens.push(Token::new(TokenType::LessEqual, self.line)); self.index += 1; },
                        _ => tokens.push(Token::new(TokenType::Less, self.line))
                    }
                },
                '>' => {
                    match self.peek(1) {
                        Some('=') => { tokens.push(Token::new(TokenType::GreaterEqual, self.line)); self.index += 1; },
                        _ => tokens.push(Token::new(TokenType::Greater, self.line))
                    }
                },
                '"' => {
                    let string_literal = self.scan_string_literal();
                    tokens.push(Token::new(TokenType::String(string_literal), self.line));
                },
                _ => {},
            }
            self.index += 1;
        }
        tokens.push(Token::new(TokenType::Eof, self.line));
        return tokens;
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rstest::*;

    #[test]
    fn test_token_type_equality() {
        assert_eq!(
            TokenType::String("Hello world!".to_string()), 
            TokenType::String("Hello world!".to_string()));

        assert_ne!(
            TokenType::String("Hello world!".to_string()), 
            TokenType::String("Hello not world!".to_string()));
    }

    #[test]
    fn test_eof_false() {
        let source = SourceCode::new("main() { return 0; }".to_string());
        assert_eq!(source.eof(), false);
    }

    #[test]
    fn test_eof_true() {
        let mut source = SourceCode::new("main() { return 0; }".to_string());
        source.index = 40;
        assert_eq!(source.eof(), true);
    }

    #[test]
    fn test_scan_tokens_multiple_tokens() {
        let mut source = SourceCode::new("+  - /".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Plus, 1), 
            Token::new(TokenType::Minus, 1), 
            Token::new(TokenType::Slash, 1),
            Token::new(TokenType::Eof, 1)]);
    }

    #[rstest]
    #[case("+", vec![Token::new(TokenType::Plus, 1)])]
    #[case("!=", vec![Token::new(TokenType::BangEqual, 1)])]
    #[case("!", vec![Token::new(TokenType::Bang, 1)])]
    #[case("==", vec![Token::new(TokenType::EqualEqual, 1)])]
    #[case("=", vec![Token::new(TokenType::Equal, 1)])]
    #[case("<=", vec![Token::new(TokenType::LessEqual, 1)])]
    #[case("<", vec![Token::new(TokenType::Less, 1)])]
    #[case(">=", vec![Token::new(TokenType::GreaterEqual, 1)])]
    #[case(">", vec![Token::new(TokenType::Greater, 1)])]
    fn test_scan_tokens_single_token(#[case] raw_source: String, #[case] expected_tokens: Vec<Token>) {
        let mut source_code = SourceCode::new(raw_source);
        let tokens = source_code.scan_tokens();
        assert_eq!(tokens[0], expected_tokens[0]);
        assert_eq!(tokens[1], Token::new(TokenType::Eof, 1))
    }

    #[test]
    fn test_scan_tokens_comment() {
        let mut source = SourceCode::new("+ == // **\n!".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::Bang, 2),
            Token::new(TokenType::Eof, 2)]);
    }
}
