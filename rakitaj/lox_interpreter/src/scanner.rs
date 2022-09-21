use crate::{tokens::{Token, TokenType}, core::location::Location};
use std::iter::{once, Peekable};

pub struct SourceCode {
    pub source: String,
    pub line: usize,
    pub filename: String
}

pub fn scan_number(initial_char: char, indices: &mut Peekable<std::str::CharIndices>) -> f32 {
    let mut floats: Vec<char> = vec![initial_char];
    while let Some(pair) = indices.peek() {
        match pair {
            (.., '0' ..= '9' | '.') => {
                floats.push(pair.1);
                indices.next();
            },
            _ => break
        }
    }
    return floats.iter().collect::<String>().parse::<f32>().unwrap();
}

pub fn is_valid_for_identifier(c: char) -> bool {
    match c {
        'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' | '?' => true,
        _ => false
    }
}

pub fn identifier_or_keyword_to_tokentype(identifier: String) -> TokenType {
    match identifier.as_str() {
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
        _ => TokenType::Identifier(identifier),
    }
}

impl SourceCode {
    pub fn new(source: String, filename: String) -> Self {
        SourceCode {
            source,
            line: 1,
            filename
        }
    }

    pub fn loc(&self) -> Location {
        Location::Line(self.filename.clone(), self.line)
    }

    pub fn peek_match_and_add(
        &self,
        indices: &mut Peekable<std::str::CharIndices>, 
        match_char: char,
        match_token_type: TokenType,
        not_match_token_type: TokenType,
        tokens: &mut Vec<Token>) {
            match indices.peek() {
                Some(pair) if pair.1 == match_char => {
                    tokens.push(Token::new(match_token_type, self.loc()));
                    indices.next();
                },
                Some(_) | None => {
                    tokens.push(Token::new(not_match_token_type, self.loc()));
                }
            }
        }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut indices = self.source.char_indices().peekable();
        while let Some((_i, c)) = indices.next() {
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
                    match indices.peek() {
                        Some(indice) if indice.1 == '/' => {
                            indices.find(|x| x.1 == '\n');
                            self.line +=1 ;
                        },
                        _ => tokens.push(Token::new(TokenType::Slash, location))
                    }
                },
                '!' => self.peek_match_and_add(&mut indices, '=', TokenType::BangEqual, TokenType::Bang, &mut tokens),
                '=' => self.peek_match_and_add(&mut indices, '=', TokenType::EqualEqual, TokenType::Equal, &mut tokens),
                '<' => self.peek_match_and_add(&mut indices, '=', TokenType::LessEqual, TokenType::Less, &mut tokens),
                '>' => self.peek_match_and_add(&mut indices, '=', TokenType::GreaterEqual, TokenType::Greater, &mut tokens),
                '0' ..= '9' => {
                    let number = scan_number(c, &mut indices);
                    tokens.push(Token::new(TokenType::Number(number), location));
                },
                '"' => {
                    let string_literal: String = indices.by_ref().take_while(|x| x.1 != '"').map(|x| { x.1 }).collect();
                    tokens.push(Token::new(TokenType::String(string_literal), location))
                },
                'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                    // Handle either an identifier or keyword.
                    // Read all the chars needed to determine if the token is a keyword or identifier.
                    let rest_of_chars = 
                        indices.by_ref()
                        .take_while(|x| is_valid_for_identifier(x.1))
                        .map(|x| x.1);
                    
                    let unknown_token = once(c).chain(rest_of_chars).collect();
                    let token_type = identifier_or_keyword_to_tokentype(unknown_token);
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
        let mut source = SourceCode::new("+  - /".to_string(), "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Plus, loc(1)), 
            Token::new(TokenType::Minus, loc(1)), 
            Token::new(TokenType::Slash, loc(1)),
            Token::new(TokenType::Eof, loc(1))]);
    }

    #[test]
    fn test_scan_tokens_comment() {
        let mut source = SourceCode::new("+ == // **\n!!".to_string(), "unittest.lox".to_string());
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
        let mut source = SourceCode::new("\"Hello world!\" ;".to_string(), "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::String("Hello world!".to_string()), loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ]);
    }

    #[test]
    fn test_scan_number_literal() {
        let mut source = SourceCode::new("\"string 123.0\" 123.0;".to_string(), "unittest.lox".to_string());
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
        let mut source = SourceCode::new("123.0\"string 123.0\" ;".to_string(), "unittest.lox".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Number(123.0), loc(1)),
            Token::new(TokenType::String("string 123.0".to_string()), loc(1)),
            Token::new(TokenType::SemiColon, loc(1)),
            Token::new(TokenType::Eof, loc(1))
        ]);
    }

    #[test]
    fn test_hello_world_line() {
        let mut source = SourceCode::new("var string = \"Hello world!\";".to_string(), "unittest.lox".to_string());
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
        let mut source = SourceCode::new("(1 + 2) * 3 \n == 9".to_string(), "unittest.lox".to_string());
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
}
