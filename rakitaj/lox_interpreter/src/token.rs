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
    pub line: usize,
}

pub fn match_peek(indices: &mut std::iter::Peekable<std::str::CharIndices>, match_char: char) -> bool {
    let peek_result = indices.peek();
    match peek_result {
        Some(pr) => {
            return pr.1 == match_char;
        },
        None => return false
    }
}

fn take_until(indices: &mut std::iter::Peekable<std::str::CharIndices>, match_char: char) -> usize {
    let mut count = 0;
    while let Some(c) = indices.next() {
        count += 1;
        if c.1 == match_char {
            break;
        }
    }
    return count;
}

pub fn scan_string_literal(indices: &mut std::iter::Peekable<std::str::CharIndices>) -> String {
    "foo".to_string()
}


impl SourceCode {
    pub fn new(source: String) -> Self {
        SourceCode {
            source: source,
            line: 1,
        }
    }

    pub fn peek_match_and_add(
        &self,
        indices: &mut std::iter::Peekable<std::str::CharIndices>, 
        match_char: char,
        match_token_type: TokenType,
        not_match_token_type: TokenType,
        tokens: &mut Vec<Token>
        ) -> () {
            let peek_matches = match_peek(indices, match_char);
            if peek_matches {
                tokens.push(Token::new(match_token_type, self.line));
                indices.next();
            } else {
                tokens.push(Token::new(not_match_token_type, self.line));
            }
        }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut indices = self.source.char_indices().peekable();
        while let Some((i, c)) = indices.next() {
            match c {
                ' ' => {},
                '\t' => {},
                '\n' => self.line += 1,
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
                    if match_peek(&mut indices, '/') {
                        take_until(&mut indices, '\n');
                        self.line +=1 ;
                    } else {
                        tokens.push(Token::new(TokenType::Slash, self.line));
                    }
                },
                '!' => self.peek_match_and_add(&mut indices, '=', TokenType::BangEqual, TokenType::Bang, &mut tokens),
                '=' => self.peek_match_and_add(&mut indices, '=', TokenType::EqualEqual, TokenType::Equal, &mut tokens),
                '<' => self.peek_match_and_add(&mut indices, '=', TokenType::LessEqual, TokenType::Less, &mut tokens),
                '>' => self.peek_match_and_add(&mut indices, '=', TokenType::GreaterEqual, TokenType::Greater, &mut tokens),
                '"' => tokens.push(Token::new(TokenType::String(scan_string_literal(&mut indices)), self.line)),
                _ => {},
            }
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

    #[test]
    fn test_scan_string_literal() {
        let mut source = SourceCode::new("\"Hello world!\"".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::String("Hello world!".to_string()), 1)
        ]);
    }
}
