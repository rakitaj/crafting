use crate::iter_extensions::TakeWhileExclusiveable;
use crate::token::get_keyword;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;

pub struct SourceContext {
    source: String,
    line: i32,
    pub had_error: bool
}

// TODO: track offset incrementally to derive column in line
fn report_error(line: i32, offset: usize, message: String) {
    println!("[line {}, offset {}] Error: {}", line, offset, message);
}

pub fn new_source(program: String) -> SourceContext {
    SourceContext {
        source: program,
        line: 0,
        had_error: false,
    }
}

pub fn tokenize(source: SourceContext) -> bool {
    let mut ctx = source;
    let mut curr = ctx.source.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((idx, c)) = curr.next() {
        let token = match c {
            // Basics
            '(' => Some(Token::new(TokenType::LeftParen, "(".to_string(), ctx.line, None)),
            ')' => Some(Token::new(TokenType::RightParen, ")".to_string(), ctx.line, None)),
            '{' => Some(Token::new(TokenType::LeftBrace, "{".to_string(), ctx.line, None)),
            '}' => Some(Token::new(TokenType::RightBrace, "}".to_string(), ctx.line, None)),
            ',' => Some(Token::new(TokenType::Comma, ",".to_string(), ctx.line, None)),
            '.' => Some(Token::new(TokenType::Dot, ".".to_string(), ctx.line, None)),
            '_' => Some(Token::new(TokenType::Minus, "_".to_string(), ctx.line, None)),
            '+' => Some(Token::new(TokenType::Plus, "+".to_string(), ctx.line, None)),
            ';' => Some(Token::new(TokenType::Semicolon, ";".to_string(), ctx.line, None)),
            '*' => Some(Token::new(TokenType::Star, "*".to_string(), ctx.line, None)),

            // Lookaheads
            '!' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        Some(Token::new(TokenType::BangEqual, "!=".to_string(), ctx.line, None))
                    }
                    _ => Some(Token::new(TokenType::Bang, "!".to_string(), ctx.line, None)),
                }
            }
            '=' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        Some(Token::new(TokenType::EqualEqual, "==".to_string(), ctx.line, None))
                    }
                    _ => Some(Token::new(TokenType::Equal, "=".to_string(), ctx.line, None)),
                }
            }
            '<' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        Some(Token::new(TokenType::LessEqual, "<=".to_string(), ctx.line, None))
                    }
                    _ => Some(Token::new(TokenType::Less, "<".to_string(), ctx.line, None)),
                }
            }
            '>' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        Some(Token::new(TokenType::GreaterEqual, ">=".to_string(), ctx.line, None))
                    }
                    _ => Some(Token::new(TokenType::Greater, ">".to_string(), ctx.line, None)),
                }
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
                        None
                    }
                    _ => Some(Token::new(TokenType::Slash, "/".to_string(), ctx.line, None)),
                }
            }

            // Misc
            ' ' | '\r' | '\t' => { None }
            '\n' => { ctx.line += 1; None },
            '0'..='9' => {
                let rest: String = (&mut curr)
                    .take_while_exclusive(|(_, d)| d.is_numeric() || *d == '.')
                    .map(|(_, d)| d)
                    .collect();
                let lexeme = format!("{}{}", c, rest);
                let number: f64 = lexeme.parse().unwrap();
                Some(Token::new(TokenType::Number, lexeme, ctx.line, Some(Literal::NumberLiteral(number))))
            }
            '"' => {
                let literal: String = (&mut curr)
                    .take_while_exclusive(|(_, s)| *s != '"' && *s != '\n')
                    .map(|(_, s)| s)
                    .collect();
                if let Some(next) = curr.next() {
                    match next {
                        (_, '\n') => {
                            ctx.line += 1;
                            ctx.had_error = true;
                            report_error(ctx.line, next.0, format!("Unterminated string"));
                            None
                        }
                        _ => Some(
                            Token::new(
                                TokenType::StringLiteral,
                                format!("\"{}\"", literal),
                                ctx.line,
                                Some(Literal::StringLiteral(literal))
                            )
                        )
                    }
                } else {
                    None
                }
            }
            'a'..='z' | 'A'..='Z' => {
                let rest: String = (&mut curr)
                    .take_while_exclusive(|(_, s)| s.is_alphanumeric())
                    .map(|(_, s)| s)
                    .collect();
                let lexeme = format!("{}{}", c, rest);
                Some(
                    Token::new(
                        get_keyword(lexeme.clone()),
                        lexeme.clone(),
                        ctx.line,
                        Some(Literal::IdentifierLiteral(lexeme))
                    )
                )
            }
            _ => {
                ctx.had_error = true;
                report_error(ctx.line, idx, format!("Unexpected character {}", c));
                None
            }
        };
        match token {
            Some(token) => { tokens.push(token) }
            _ => {}
        };
    }

    tokens.push(Token::new(TokenType::EOF, "".to_string(), ctx.line, None));

    for t in tokens {
        println!("[x] {} [t] {:?} [l] {}", t.lexeme, t.token_type, t.line);
    };

    ctx.had_error
}
