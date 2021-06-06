use crate::iter_extensions::TakeUntilable;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;

pub struct SourceContext {
    source: String,
    idx: usize,
    line: i32,
}

pub fn new_source(program: String) -> SourceContext {
    SourceContext {
        source: program,
        idx: 0,
        line: 0,
    }
}

pub fn tokenize(source: SourceContext) {
    scan_token(source)
}

fn scan_token(source: SourceContext) {
    let mut ctx = source;
    let mut curr = ctx.source.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((idx, c)) = curr.next() {
        match c {
            // Basics
            '(' => tokens.push(Token::new(TokenType::LeftParen, "(".to_string(), ctx.line, None)),
            ')' => tokens.push(Token::new(TokenType::RightParen, ")".to_string(), ctx.line, None)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, "{".to_string(), ctx.line, None)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, "}".to_string(), ctx.line, None)),
            ',' => tokens.push(Token::new(TokenType::Comma, ",".to_string(), ctx.line, None)),
            '.' => tokens.push(Token::new(TokenType::Dot, ".".to_string(), ctx.line, None)),
            '_' => tokens.push(Token::new(TokenType::Minus, "_".to_string(), ctx.line, None)),
            '+' => tokens.push(Token::new(TokenType::Plus, "+".to_string(), ctx.line, None)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, ";".to_string(), ctx.line, None)),
            '*' => tokens.push(Token::new(TokenType::Star, "*".to_string(), ctx.line, None)),

            // Lookaheads
            '!' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::BangEqual, "!=".to_string(), ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Bang, "!".to_string(), ctx.line, None)),
                };
            }
            '=' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::EqualEqual, "==".to_string(), ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Equal, "=".to_string(), ctx.line, None)),
                };
            }
            '<' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Less, "<".to_string(), ctx.line, None)),
                };
            }
            '>' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Greater, ">".to_string(), ctx.line, None)),
                };
            }
            '/' => {
                match curr.peek() {
                    Some((_, '/')) => {
                        curr.next();
                        while let Some((_, next_c)) = curr.peek() {
                            match next_c {
                                '\n' => break,
                                _ => { curr.next(); continue; }
                            }
                        }
                    }
                    _ => tokens.push(Token::new(TokenType::Slash, "/".to_string(), ctx.line, None)),
                };
            }

            // Misc
            ' ' | '\r' | '\t' => {}
            '\n' => ctx.line += 1,
            '0'..='9' => {
                let rest: String = (&mut curr)
                    .take_until(|(_, d)| d.is_digit(10) || *d == '.')
                    .map(|(_, d)| d)
                    .collect();
                let lexeme: String = format!("{}{}", c, rest);
                let number: f64 = lexeme.parse().unwrap();

                tokens.push(Token::new(TokenType::Number, lexeme, ctx.line, Some(Literal::NumberLiteral(number))));
            }
            _ => {
                ctx.idx = idx;
                // println!("DEFAULT {} at {}", c, idx)
                print!("{}", c);
            }
        }
    }

    for t in tokens {
        println!("t: {}", t.lexeme);
    }

    println!("");
}
