use crate::{tokens::{Token, TokenType}, core::location::Location};
use std::iter::Peekable;

pub struct SourceCode<'a> {
    pub source: String,
    pub line: usize,
    pub filename: String,
    indices: Peekable<std::str::CharIndices<'a>>
}

pub fn is_valid_for_identifier(c: char) -> bool {
    match c {
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' | '?' => true,
        _ => false
    }
}

pub fn is_valid_for_number(c: char) -> bool {
    match c {
        '0' ..= '9' | '.' => true,
        _ => false
    }
}

pub fn identifier_or_keyword_to_tokentype(identifier: &str) -> TokenType {
    match identifier {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "fun" => TokenType::Fun,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier(identifier.to_string()),
    }
}

impl<'a> SourceCode<'a> {
    pub fn new(source: &'a str, filename: String) -> Self {
        SourceCode {
            source: source.to_string(),
            line: 1,
            filename,
            indices: source.char_indices().peekable()
        }
    }

    pub fn loc(&self) -> Location {
        Location::Line(self.filename.clone(), self.line)
    }

    pub fn take_while_inclusive<F: Fn(char) -> bool>(&mut self, pred: F, initial: (usize, char)) -> (usize, usize) {
        let mut current = initial.0;
        while let Some(x) = self.indices.peek() {
            if pred(x.1) {
                current = x.0;
                self.indices.next();                
            } else {
                break
            }
        }
        (initial.0, current + 1)
    }

    pub fn peek_match_and_add(
        &mut self,
        match_char: char,
        match_token_type: TokenType,
        not_match_token_type: TokenType,
        tokens: &mut Vec<Token>) {
            match self.indices.peek() {
                Some(pair) if pair.1 == match_char => {
                    tokens.push(Token::new(match_token_type, self.loc()));
                    self.indices.next();
                },
                Some(_) | None => {
                    tokens.push(Token::new(not_match_token_type, self.loc()));
                }
            }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some((i, c)) = self.indices.next() {
            let location = self.loc();
            match c {
                ' ' => {},
                '\t' => {},
                '\n' => self.line += 1,
                '(' => tokens.push(Token::new(TokenType::LeftParen, location)),
                ')' => tokens.push(Token::new(TokenType::RightParen, location)),
                '{' => tokens.push(Token::new(TokenType::LeftBrace, location)),
                '}' => tokens.push(Token::new(TokenType::RightBrace, location)),
                ',' => tokens.push(Token::new(TokenType::Comma, location)),
                '.' => tokens.push(Token::new(TokenType::Dot, location)),
                '-' => tokens.push(Token::new(TokenType::Minus, location)),
                '+' => tokens.push(Token::new(TokenType::Plus, location)),
                ';' => tokens.push(Token::new(TokenType::SemiColon, location)),
                '*' => tokens.push(Token::new(TokenType::Star, location)),
                '/' => {
                    match self.indices.peek() {
                        Some(indice) if indice.1 == '/' => {
                            self.indices.find(|x| x.1 == '\n');
                            self.line +=1 ;
                        },
                        _ => tokens.push(Token::new(TokenType::Slash, location))
                    }
                },
                '!' => self.peek_match_and_add('=', TokenType::BangEqual, TokenType::Bang, &mut tokens),
                '=' => self.peek_match_and_add('=', TokenType::EqualEqual, TokenType::Equal, &mut tokens),
                '<' => self.peek_match_and_add('=', TokenType::LessEqual, TokenType::Less, &mut tokens),
                '>' => self.peek_match_and_add('=', TokenType::GreaterEqual, TokenType::Greater, &mut tokens),
                '0' ..= '9' => {
                    let (start, end) = self.take_while_inclusive(is_valid_for_number, (i, c));
                    let number = self.source[start..end].parse::<f32>().unwrap();
                    tokens.push(Token::new(TokenType::Number(number), location));
                },
                '"' => {
                    let string_literal: String = self.indices.by_ref().take_while(|x| x.1 != '"').map(|x| { x.1 }).collect();
                    tokens.push(Token::new(TokenType::String(string_literal), location))
                },
                'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                    let (start, end) = self.take_while_inclusive(is_valid_for_identifier, (i, c));
                    let token_lexeme = &self.source[start..end];
                    let token_type = identifier_or_keyword_to_tokentype(token_lexeme);
                    tokens.push(Token::new(token_type, location));
                }
                _ => {},
            }
        }
        tokens.push(Token::new(TokenType::Eof, self.loc()));
        tokens
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn loc(line: usize) -> Location {
        Location::Line("unittest.lox".to_string(), line)
    }

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
        let mut source = SourceCode::new("+  - /", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Plus, loc(1)), 
            Token::new(TokenType::Minus, loc(1)), 
            Token::new(TokenType::Slash, loc(1)),
            Token::new(TokenType::Eof, loc(1))]);
    }

    #[test]
    fn test_scan_tokens_comment() {
        let mut source = SourceCode::new("+ == // **\n!!", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Plus, loc(1)),
            Token::new(TokenType::EqualEqual, loc(1)),
            Token::new(TokenType::Bang, loc(2)),
            Token::new(TokenType::Bang, loc(2)),
            Token::new(TokenType::Eof, loc(2))]);
    }

    #[test]
    fn test_scan_string_literal() {
        let mut source = SourceCode::new("\"Hello world!\" ;", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::String("Hello world!".to_string()), loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ]);
    }

    #[test]
    fn test_scan_number_literal() {
        let mut source = SourceCode::new("\"string 123.0\" 123.0;", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::String("string 123.0".to_string()), loc(1)),
            Token::new(TokenType::Number(123.0), loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ]);
    }

    #[test]
    fn test_scan_number_literal_2() {
        let mut source = SourceCode::new("12.3+0.0;", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Number(12.3), loc(1)),
            Token::new(TokenType::Plus, loc(1)),
            Token::new(TokenType::Number(0.0), loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ]);
    }

    #[test]
    fn test_hello_world_line() {
        let mut source = SourceCode::new("var string = \"Hello world!\";", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Var, loc(1)),
            Token::new(TokenType::Identifier("string".to_string()), loc(1)),
            Token::new(TokenType::Equal, loc(1)),
            Token::new(TokenType::String("Hello world!".to_string()), loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ]);
    }

    #[test]
    fn test_scan_math_expression() {
        let mut source = SourceCode::new("(1+2) * 3 \n == 9", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::LeftParen, loc(1)),
            Token::new(TokenType::Number(1f32), loc(1)),
            Token::new(TokenType::Plus, loc(1)),
            Token::new(TokenType::Number(2f32), loc(1)),
            Token::new(TokenType::RightParen, loc(1)),
            Token::new(TokenType::Star, loc(1)),
            Token::new(TokenType::Number(3f32), loc(1)),
            Token::new(TokenType::EqualEqual, loc(2)),
            Token::new(TokenType::Number(9f32), loc(2)),
            Token::new(TokenType::Eof, loc(2))
        ]);
    }

    #[test]
    fn test_scan_function_declaration() {
        let mut source = SourceCode::new("fun foo(){}", "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Fun, loc(1)),
            Token::new(TokenType::Identifier("foo".to_string()), loc(1)),
            Token::new(TokenType::LeftParen, loc(1)),
            Token::new(TokenType::RightParen, loc(1)),
            Token::new(TokenType::LeftBrace, loc(1)),
            Token::new(TokenType::RightBrace, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ]);
    }

    #[test]
    fn test_scan_function_body() {
        let mut source = SourceCode::new(
r#"fun underscores_are_valid() {
    // comment on a line
    var foo = "baz"; // comment after line
    (1+2) }
        "#, "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Fun, loc(1)),
            Token::new(TokenType::Identifier("underscores_are_valid".to_string()), loc(1)),
            Token::new(TokenType::LeftParen, loc(1)),
            Token::new(TokenType::RightParen, loc(1)),
            Token::new(TokenType::LeftBrace, loc(1)),
            Token::new(TokenType::Var, loc(3)),
            Token::new(TokenType::Identifier("foo".to_string()), loc(3)),
            Token::new(TokenType::Equal, loc(3)),
            Token::new(TokenType::String("baz".to_string()), loc(3)),
            Token::new(TokenType::SemiColon, loc(3)),
            Token::new(TokenType::LeftParen, loc(4)),
            Token::new(TokenType::Number(1.0), loc(4)),
            Token::new(TokenType::Plus, loc(4)),
            Token::new(TokenType::Number(2.0), loc(4)),
            Token::new(TokenType::RightParen, loc(4)),
            Token::new(TokenType::RightBrace, loc(4)),
            Token::new(TokenType::Eof, loc(5))
        ]);
    }
}
