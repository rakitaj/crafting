use std::fmt;
use crate::tokens::TokenType;

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

pub fn scan_number(initial_char: char, indices: &mut std::iter::Peekable<std::str::CharIndices>) -> f32 {
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
        'a' ..= 'z' | 'A' ..= 'Z' => true,
        '0' ..= '9' => true,
        '_' | '?' => true,
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
        while let Some((_i, c)) = indices.next() {
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
                        indices.by_ref().skip_while(|x| x.1 != '\n').next();
                        self.line +=1 ;
                    } else {
                        tokens.push(Token::new(TokenType::Slash, self.line));
                    }
                },
                '!' => self.peek_match_and_add(&mut indices, '=', TokenType::BangEqual, TokenType::Bang, &mut tokens),
                '=' => self.peek_match_and_add(&mut indices, '=', TokenType::EqualEqual, TokenType::Equal, &mut tokens),
                '<' => self.peek_match_and_add(&mut indices, '=', TokenType::LessEqual, TokenType::Less, &mut tokens),
                '>' => self.peek_match_and_add(&mut indices, '=', TokenType::GreaterEqual, TokenType::Greater, &mut tokens),
                '0' ..= '9' => {
                    let number = scan_number(c, &mut indices);
                    tokens.push(Token::new(TokenType::Number(number), self.line));
                },
                '"' => {
                    let string_as_chars: Vec<(usize, char)> = indices.by_ref().take_while(|x| x.1 != '"').collect();
                    let string_literal: String = string_as_chars.into_iter().map(|x| { x.1 }).collect();
                    tokens.push(Token::new(TokenType::String(string_literal), self.line))
                },
                'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                    // Handle either an identifier or keyword.
                    // Read all the chars needed to determine if the token is a keyword or identifier.
                    let mut ambiguous_token = vec![c];
                    let rest_of_chars: Vec<(usize, char)> = indices.by_ref().take_while(|x| is_valid_for_identifier(x.1)).collect();
                    ambiguous_token.extend(rest_of_chars.into_iter().map(|x| x.1));

                    let token_type = identifier_or_keyword_to_tokentype(ambiguous_token.into_iter().collect());
                    tokens.push(Token::new(token_type, self.line));
                }
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
    #[case("(", vec![Token::new(TokenType::LeftParen, 1)])]
    #[case(")", vec![Token::new(TokenType::RightParen, 1)])]
    #[case("{", vec![Token::new(TokenType::LeftBrace, 1)])]
    #[case("}", vec![Token::new(TokenType::RightBrace, 1)])]

    #[case(",", vec![Token::new(TokenType::Comma, 1)])]
    #[case(".", vec![Token::new(TokenType::Dot, 1)])]
    #[case("-", vec![Token::new(TokenType::Minus, 1)])]
    #[case("+", vec![Token::new(TokenType::Plus, 1)])]
    #[case(";", vec![Token::new(TokenType::SemiColon, 1)])]
    #[case("/", vec![Token::new(TokenType::Slash, 1)])]
    #[case("*", vec![Token::new(TokenType::Star, 1)])]


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
        let mut source = SourceCode::new("+ == // **\n!!".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Plus, 1),
            Token::new(TokenType::EqualEqual, 1),
            Token::new(TokenType::Bang, 2),
            Token::new(TokenType::Bang, 2),
            Token::new(TokenType::Eof, 2)]);
    }

    #[test]
    fn test_scan_string_literal() {
        let mut source = SourceCode::new("\"Hello world!\" ;".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::String("Hello world!".to_string()), 1),
            Token::new(TokenType::SemiColon, 1),
            Token::new(TokenType::Eof, 1)
        ]);
    }

    #[test]
    fn test_scan_number_literal() {
        let mut source = SourceCode::new("\"string 123.0\" 123.0;".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::String("string 123.0".to_string()), 1),
            Token::new(TokenType::Number(123.0), 1),
            Token::new(TokenType::SemiColon, 1),
            Token::new(TokenType::Eof, 1)
        ]);
    }

    #[test]
    fn test_scan_number_literal_2() {
        let mut source = SourceCode::new("123.0\"string 123.0\" ;".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Number(123.0), 1),
            Token::new(TokenType::String("string 123.0".to_string()), 1),
            Token::new(TokenType::SemiColon, 1),
            Token::new(TokenType::Eof, 1)
        ]);
    }

    #[test]
    fn test_hello_world_line() {
        let mut source = SourceCode::new("var string = \"Hello world!\";".to_string());
        let tokens = source.scan_tokens();
        assert_eq!(tokens, vec![
            Token::new(TokenType::Var, 1),
            Token::new(TokenType::Identifier("string".to_string()), 1),
            Token::new(TokenType::Equal, 1),
            Token::new(TokenType::String("Hello world!".to_string()), 1),
            Token::new(TokenType::SemiColon, 1),
            Token::new(TokenType::Eof, 1)
        ]);
    }
}
