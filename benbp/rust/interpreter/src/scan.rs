use crate::token::Token;
use crate::token::TokenType;

pub struct SourceContext {
    source: String,
    idx: usize,
    line: i32,
}

impl SourceContext {
    fn next() -> char {
        let foo: char = 'a';
        foo
    }
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
            '(' => tokens.push(Token::new(TokenType::LeftParen, "(", ctx.line, None)),
            ')' => tokens.push(Token::new(TokenType::RightParen, ")", ctx.line, None)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, "{", ctx.line, None)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, "}", ctx.line, None)),
            ',' => tokens.push(Token::new(TokenType::Comma, ",", ctx.line, None)),
            '.' => tokens.push(Token::new(TokenType::Dot, ".", ctx.line, None)),
            '_' => tokens.push(Token::new(TokenType::Minus, "_", ctx.line, None)),
            '+' => tokens.push(Token::new(TokenType::Plus, "+", ctx.line, None)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, ";", ctx.line, None)),
            '*' => tokens.push(Token::new(TokenType::Star, "*", ctx.line, None)),

            // Add peek for combos
            '!' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next(); // advance iterator past lexeme
                        tokens.push(Token::new(TokenType::BangEqual, "!=", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Bang, "!", ctx.line, None)),
                };
            }
            '=' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next(); // advance iterator past lexeme
                        tokens.push(Token::new(TokenType::Equal, "==", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Equal, "=", ctx.line, None)),
                };
            }
            '<' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next(); // advance iterator past lexeme
                        tokens.push(Token::new(TokenType::LessEqual, "<=", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Less, "<", ctx.line, None)),
                };
            }
            '>' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next(); // advance iterator past lexeme
                        tokens.push(Token::new(TokenType::GreaterEqual, ">=", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Greater, ">", ctx.line, None)),
                };
            }
            '/' => {
                match curr.peek() {
                    Some((_, '/')) => {
                        curr.next(); // advance iterator past lexeme
                        while let Some((_, next_c)) = curr.peek() {
                            match next_c {
                                '\n' => break,
                                _ => {
                                    curr.next();
                                    continue;
                                }
                            }
                        }
                    }
                    _ => tokens.push(Token::new(TokenType::Slash, "/", ctx.line, None)),
                };
            }

            '\n' => ctx.line += 1,
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
