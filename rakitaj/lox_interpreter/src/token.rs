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
    line: usize,
    lexeme: String,
    literal: Option<String>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        lexeme: String,
        literal: Option<String>,
    ) -> Self {
        Token {
            token_type: token_type,
            line: line,
            lexeme: lexeme,
            literal: literal,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            &self.token_type,
            &self.lexeme,
            &self.literal.as_ref().unwrap_or(&"No literal".to_string())
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

    pub fn eof(&self) -> bool {
        match &self.get() {
            Some(_) => false,
            None => true,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(c) = self.get() {
            let start = self.index;
            self.index += 1;
            match c {
                ' ' => continue,
                '\t' => continue,
                '\n' => {
                    self.line += 1;
                    continue;
                }
                '(' => tokens.push(Token::new(TokenType::LeftParen, self.line, self.source[start..self.index].to_string(), None)),
                ')' => tokens.push(Token::new(TokenType::RightParen, self.line, self.source[start..self.index].to_string(), None)),
                '{' => tokens.push(Token::new(TokenType::LeftBrace, self.line, self.source[start..self.index].to_string(), None)),
                '}' => tokens.push(Token::new(TokenType::RightBrace, self.line, self.source[start..self.index].to_string(), None)),
                ',' => tokens.push(Token::new(TokenType::Comma, self.line, self.source[start..self.index].to_string(), None)),
                '.' => tokens.push(Token::new(TokenType::Dot, self.line, self.source[start..self.index].to_string(), None)),
                '-' => tokens.push(Token::new(TokenType::Minus, self.line, self.source[start..self.index].to_string(), None)),
                '+' => tokens.push(Token::new(TokenType::Plus, self.line, self.source[start..self.index].to_string(), None)),
                ';' => tokens.push(Token::new(TokenType::SemiColon, self.line, self.source[start..self.index].to_string(), None)),
                '*' => tokens.push(Token::new(TokenType::Star, self.line, self.source[start..self.index].to_string(), None)),
                '/' => tokens.push(Token::new(TokenType::Slash, self.line, self.source[start..self.index].to_string(), None)),
                _ => continue,
            }
        }
        return tokens;
    }
}

pub fn lex(mut source: SourceCode) -> Vec<Token> {
    return source.scan_tokens();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rstest::*;

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
    fn test_scan_tokens_single_token() {
        let mut source = SourceCode::new("+".to_string());
        let tokens = source.scan_tokens();
        let token = Token::new(TokenType::Plus, 1, "+".to_string(), None);
        assert_eq!(tokens, vec![token]);
    }

    #[test]
    fn test_scan_tokens_multiple_tokens() {
        let mut source = SourceCode::new("+  - /".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Plus, 1, "+".to_string(), None), 
            Token::new(TokenType::Minus, 1, "-".to_string(), None), 
            Token::new(TokenType::Slash, 1, "/".to_string(), None)]);
    }

    // #[rstest]
    // #[case("!=", Some(TokenType::BangEqual))]
    // #[case("!", Some(TokenType::Bang))]
    // #[case("<=", Some(TokenType::LessEqual))]
    // #[case("<", Some(TokenType::Less))]
    // #[case(">=", Some(TokenType::GreaterEqual))]
    // #[case(">", Some(TokenType::Greater))]
    // fn test_next_token(#[case] raw_source: String, #[case] expected_token: Option<TokenType>) {
    //     let mut source_code = SourceCode::new(raw_source);
    //     let token = source_code.next_token();
    //     assert_eq!(token, expected_token);
    // }
}
